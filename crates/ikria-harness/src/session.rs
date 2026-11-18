//! A bounded single-vessel flight session with write-ahead command records.
//! Transport failures are terminal. Commands with an uncertain outcome are not retried.
//!
//! Every frame read from the bridge is journalled before it is validated, so a refused
//! sample remains evidence. A normal-program output that the session refuses, whether
//! by phase sequencing, the completion guard or the authorization policy, is treated as
//! a program fault. The refused command is never sent, and the precompiled abort program
//! answers the same sample. A refusal of the abort program's own output ends the session.
use crate::flight::{self as program, FlightOutput, FlightProgram};
use ikria_records::flight::{self, BridgeFrame, Commanded, HarnessFrame, Telemetry};
use ikria_records::{GameTime, PartId, Situation, VesselCommand};
use keleusma::Arena;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    io::{BufRead, Write},
    path::Path,
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
/// The abort program is the bundled [`program::ABORT`], a source distinct from the
/// normal program.
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
            abort: program::ABORT,
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

/// Session constants that every command is checked against.
struct Profile {
    engine: PartId,
    chute: PartId,
    armed: bool,
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
    let profile = Profile {
        engine,
        chute,
        armed,
    };
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
    let mut commanded = Commanded::default();
    for expected in 0..max_ticks {
        // Journal before validation, so that a refused sample is still evidence.
        let frame = read_logged(reader, log)?;
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
        if mode != Mode::Monitor && !aborting && abort_requested() {
            aborting = true;
            flight::write_frame(
                log,
                &json!({"event":"abort_requested","seq":seq,"phase":phase}),
            )?;
        }
        let start = Instant::now();
        // Programs and policy see a commanded one-shot action as done, even while the
        // telemetry flag lags. The journal keeps the raw sample.
        let view = commanded.latch(telemetry);
        let command = if mode == Mode::Monitor {
            VesselCommand {
                id: telemetry.id,
                ..VesselCommand::default()
            }
        } else {
            let normal_decision = if aborting {
                None
            } else {
                Some(
                    normal
                        .step(&view, phase, false)
                        .map_err(|e| e.to_string())
                        .and_then(|output| {
                            decide(&view, phase, false, output, &profile, commanded)
                                .map_err(|e| e.to_string())
                        }),
                )
            };
            let (next, command) = match normal_decision {
                Some(Ok(decision)) => decision,
                Some(Err(error)) => {
                    // The refused output was never sent, so routing to the abort
                    // program cannot repeat an uncertain command.
                    flight::write_frame(
                        log,
                        &json!({"event":"program_fault","seq":seq,"phase":phase,"error":error}),
                    )?;
                    aborting = true;
                    abort_decision(&mut abort_program, &view, phase, &profile, commanded)?
                }
                None => abort_decision(&mut abort_program, &view, phase, &profile, commanded)?,
            };
            phase = next;
            command
        };
        flight::authorize_latched(telemetry, &command, engine, chute, armed, commanded)?;
        // Latch before the write. A failed or unacknowledged write is uncertain, and
        // an uncertain one-shot action is never requested again.
        commanded.record(&command);
        let request = HarnessFrame::Command { seq, ut, command };
        // Log intent before sending. Missing Applied leaves the outcome unknown.
        flight::write_frame(
            log,
            &json!({"event":"command_intent","phase":phase,"aborting":aborting,
            "compute_ns":start.elapsed().as_nanos(),"request":request}),
        )?;
        flight::write_frame(writer, &request)?;
        let ack = read_logged(reader, log)?;
        if ack != (BridgeFrame::Applied { seq }) {
            return Err(format!("command outcome uncertain: {ack:?}").into());
        }
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

/// Read one frame and journal it before it is interpreted. A frame that is JSON but
/// not a bridge frame is journalled as an unparsed frame and then refused. Bytes that
/// are not JSON, or that exceed the frame bound, cannot be journalled as a record.
fn read_logged<R: BufRead, L: Write>(reader: &mut R, log: &mut L) -> Result<BridgeFrame> {
    let value: serde_json::Value = flight::read_frame(reader)?;
    match serde_json::from_value::<BridgeFrame>(value.clone()) {
        Ok(frame) => {
            flight::write_frame(log, &frame)?;
            Ok(frame)
        }
        Err(error) => {
            flight::write_frame(
                log,
                &json!({"event":"unparsed_frame","frame":value,"error":error.to_string()}),
            )?;
            Err(error.into())
        }
    }
}

/// Step the abort program and check its output. Any error here ends the session,
/// because no further fallback exists.
fn abort_decision(
    abort_program: &mut FlightProgram<'_>,
    view: &Telemetry,
    phase: i64,
    profile: &Profile,
    commanded: Commanded,
) -> Result<(i64, VesselCommand)> {
    let output = abort_program
        .step(view, phase, true)
        .map_err(|e| format!("abort program failed: {e}"))?;
    decide(view, phase, true, output, profile, commanded)
        .map_err(|e| format!("abort program output refused: {e}").into())
}

/// Check one program output against phase sequencing, the ignition rule, the abort
/// rule and the completion guard, then convert it and authorize the whole command.
/// Returns the next phase and the command. Nothing is sent here.
fn decide(
    view: &Telemetry,
    phase: i64,
    aborting: bool,
    output: FlightOutput,
    profile: &Profile,
    commanded: Commanded,
) -> Result<(i64, VesselCommand)> {
    let transition = match phase {
        0 => [1, 4, 5].contains(&output.phase),
        1 => [1, 2, 3, 4, 5].contains(&output.phase),
        2 => [2, 3, 4, 5].contains(&output.phase),
        3 => [3, 4, 5].contains(&output.phase),
        4 => [3, 4, 5].contains(&output.phase),
        5 => [4, 5].contains(&output.phase),
        _ => false,
    };
    if !transition {
        return Err(format!("invalid phase transition {phase} to {}", output.phase).into());
    }
    if output.ignite != 0 && (aborting || phase != 0) {
        return Err("ignition outside the pad phase or during abort".into());
    }
    // The abort table requires zero continuous commands. The session enforces this
    // rather than trusting the abort program.
    if aborting && [output.throttle, output.pitch, output.yaw, output.roll] != [0; 4] {
        return Err("continuous command during abort".into());
    }
    if output.phase == 4
        && !matches!(view.situation, Situation::Landed | Situation::Splashed)
        && !(aborting && phase == 0 && view.situation == Situation::PreLaunch)
    {
        return Err("completion requires touchdown or a pad abort".into());
    }
    let command = program::command(view, output, profile.engine, profile.chute)?;
    flight::authorize_latched(
        view,
        &command,
        profile.engine,
        profile.chute,
        profile.armed,
        commanded,
    )?;
    Ok((output.phase, command))
}

/// Whether an operator abort-request file is present. A path that cannot be checked,
/// for example because of a permission error, counts as a request, so that the
/// interface fails closed. No path means no request.
pub fn abort_file_requested(path: Option<&Path>) -> bool {
    path.is_some_and(|path| path.try_exists().unwrap_or(true))
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
