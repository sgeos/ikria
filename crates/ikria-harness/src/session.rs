//! A bounded single-vessel flight session with write-ahead command records.
//! Transport failures are terminal. Commands with an uncertain outcome are not retried.
use crate::flight::{self as program, FlightProgram};
use ikria_records::flight::{self, BridgeFrame, HarnessFrame, Telemetry};
use ikria_records::{GameTime, Situation, VesselCommand};
use keleusma::Arena;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    io::{BufRead, Write},
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Requested session mode. Monitoring sends only neutral commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Read telemetry without actuator writes.
    Monitor,
    /// Execute the hop in the verification save.
    Hop,
    /// Exercise the abort program before ignition.
    PadAbort,
}

/// Run against any bounded transport and append records to a caller-owned writer.
/// Callers must impose read and write deadlines. Log failure stops before the next
/// command. The caller must send Close or disconnect after every return, including
/// errors. No resume of a terminated session is supported.
pub fn run<R: BufRead, W: Write, L: Write>(
    reader: &mut R,
    writer: &mut W,
    log: &mut L,
    mode: Mode,
    max_ticks: u64,
    source: &str,
) -> Result<()> {
    run_with_abort(reader, writer, log, mode, max_ticks, source, || false)
}

/// Run with an external abort request sampled once per tick. Once observed, abort is
/// latched and the precompiled abort program owns all subsequent valid samples.
pub fn run_with_abort<R: BufRead, W: Write, L: Write, F: FnMut() -> bool>(
    reader: &mut R,
    writer: &mut W,
    log: &mut L,
    mode: Mode,
    max_ticks: u64,
    source: &str,
    abort_requested: F,
) -> Result<()> {
    run_with_sources(
        reader,
        writer,
        log,
        mode,
        max_ticks,
        ProgramSources {
            normal: source,
            abort: program::HOP,
        },
        abort_requested,
    )
}

/// Source pair used to reproduce a historical flight. Live callers use the bundled
/// independent abort program through `run_with_abort`.
pub struct ProgramSources<'a> {
    /// Normal program body.
    pub normal: &'a str,
    /// Independently verified abort program body.
    pub abort: &'a str,
}

/// Run an explicit verified program pair. Both source hashes are journalled.
pub fn run_with_sources<R: BufRead, W: Write, L: Write, F: FnMut() -> bool>(
    reader: &mut R,
    writer: &mut W,
    log: &mut L,
    mode: Mode,
    max_ticks: u64,
    sources: ProgramSources<'_>,
    mut abort_requested: F,
) -> Result<()> {
    let source = sources.normal;
    if max_ticks == 0 || max_ticks > 30000 {
        return Err("tick limit out of bounds".into());
    }
    let arena = Arena::with_capacity(1024 * 1024);
    let abort_arena = Arena::with_capacity(1024 * 1024);
    let mut normal = FlightProgram::new(source, &arena, 10000)?;
    // Abort is precompiled and independently allocated. A faulted normal VM is never
    // resumed. A fault in the abort VM ends the session.
    let mut abort_program = FlightProgram::new(sources.abort, &abort_arena, 10000)?;
    let hello: BridgeFrame = flight::read_frame(reader)?;
    let BridgeFrame::Hello {
        version,
        ref save,
        ref vessel,
        armed,
        engine,
        chute,
    } = hello
    else {
        return Err("expected bridge hello".into());
    };
    if version != flight::VERSION
        || save != flight::VERIFICATION_SAVE
        || vessel != "IKR-TS-001B"
        || armed != (mode != Mode::Monitor)
        || engine == chute
    {
        return Err("identity, version or mode mismatch".into());
    }
    flight::write_frame(
        log,
        &json!({"event":"session","hello":hello,"mode":format!("{mode:?}"),
        "program_sha256":format!("{:x}",Sha256::digest(format!("{}\n{source}",program::DIALOGUE))),
        "abort_sha256":format!("{:x}",Sha256::digest(format!("{}\n{}",program::DIALOGUE,sources.abort))),
        "nominal_cycles":normal.cycles(),"abort_nominal_cycles":abort_program.cycles()}),
    )?;
    flight::write_frame(
        writer,
        &HarnessFrame::Hello {
            version: flight::VERSION,
        },
    )?;
    let mut previous: Option<GameTime> = None;
    let mut phase = 0;
    let mut aborting = mode == Mode::PadAbort;
    let mut parts = None;
    let mut touchdown_since = None;
    for expected in 0..max_ticks {
        let frame: BridgeFrame = flight::read_frame(reader)?;
        let BridgeFrame::Sample {
            seq,
            ut,
            ref telemetry,
        } = frame
        else {
            return Err(format!("expected sample, got {frame:?}").into());
        };
        validate_sample(telemetry, seq, expected, ut, previous, parts)?;
        if expected == 0
            && mode != Mode::Monitor
            && (telemetry.situation != Situation::PreLaunch
                || telemetry.engine_active
                || telemetry.chute_deployed)
        {
            return Err("flight must begin untouched on the pad".into());
        }
        previous = Some(ut);
        parts = Some(telemetry.parts);
        flight::write_frame(log, &frame)?;
        if mode != Mode::Monitor && !aborting && abort_requested() {
            aborting = true;
            flight::write_frame(
                log,
                &json!({"event":"abort_requested","seq":seq,"phase":phase}),
            )?;
        }
        let start = Instant::now();
        let command = if mode == Mode::Monitor {
            VesselCommand {
                id: telemetry.id,
                ..VesselCommand::default()
            }
        } else {
            let output = if aborting {
                abort_program.step(telemetry, phase, true)?
            } else {
                match normal.step(telemetry, phase, false) {
                    Ok(output) => output,
                    Err(error) => {
                        flight::write_frame(
                            log,
                            &json!({"event":"program_fault","seq":seq,"error":error.to_string()}),
                        )?;
                        aborting = true;
                        abort_program.step(telemetry, phase, true)?
                    }
                }
            };
            let transition = match phase {
                0 => [1, 4, 5].contains(&output.phase),
                1 => [1, 2, 3, 4, 5].contains(&output.phase),
                2 => [2, 3, 4, 5].contains(&output.phase),
                3 => [3, 4, 5].contains(&output.phase),
                4 => [3, 4, 5].contains(&output.phase),
                5 => [4, 5].contains(&output.phase),
                _ => false,
            };
            if !transition || (aborting && output.ignite != 0) || (output.ignite == 1 && phase != 0)
            {
                return Err("invalid phase transition or ignition request".into());
            }
            if output.phase == 4
                && !matches!(telemetry.situation, Situation::Landed | Situation::Splashed)
                && !(aborting && phase == 0 && telemetry.situation == Situation::PreLaunch)
            {
                return Err("completion requires touchdown or a pad abort".into());
            }
            phase = output.phase;
            program::command(telemetry, output, engine, chute)?
        };
        flight::authorize(telemetry, &command, engine, chute, armed)?;
        let request = HarnessFrame::Command { seq, ut, command };
        // Log intent before sending. Missing Applied leaves the outcome unknown.
        flight::write_frame(
            log,
            &json!({"event":"command_intent","phase":phase,"aborting":aborting,
            "compute_ns":start.elapsed().as_nanos(),"request":request}),
        )?;
        flight::write_frame(writer, &request)?;
        let ack: BridgeFrame = flight::read_frame(reader)?;
        if ack != (BridgeFrame::Applied { seq }) {
            return Err(format!("command outcome uncertain: {ack:?}").into());
        }
        flight::write_frame(log, &ack)?;
        if phase == 4
            && ((matches!(telemetry.situation, Situation::Landed | Situation::Splashed)
                && telemetry.vertical_speed_m_s.abs() < 0.5)
                || (aborting && telemetry.situation == Situation::PreLaunch))
        {
            let since = *touchdown_since.get_or_insert(ut.seconds());
            if (aborting && telemetry.situation == Situation::PreLaunch)
                || ut.seconds() - since >= 1.0
            {
                flight::write_frame(log, &json!({"event":"complete","ut":ut,"aborted":aborting}))?;
                return Ok(());
            }
        } else {
            touchdown_since = None;
        }
    }
    if mode == Mode::Monitor {
        Ok(())
    } else {
        Err("flight did not complete within tick budget".into())
    }
}

/// Validate chronology and the physical sample before calling a program.
pub fn validate_sample(
    t: &Telemetry,
    seq: u64,
    expected: u64,
    ut: GameTime,
    previous: Option<GameTime>,
    parts: Option<u32>,
) -> Result<()> {
    t.validate()?;
    if seq != expected
        || t.id.0 != 1
        || t.crew != 0
        || !t.physics_active
        || parts.is_some_and(|p| p != t.parts)
    {
        return Err("sequence, identity, crew, physics or part count changed".into());
    }
    if previous
        .is_some_and(|p| ut.seconds() - p.seconds() < 0.001 || ut.seconds() - p.seconds() > 0.03)
    {
        return Err("missing, duplicate or reversed physics tick".into());
    }
    Ok(())
}
