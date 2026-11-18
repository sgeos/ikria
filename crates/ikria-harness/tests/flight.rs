//! Flight boundary, ordering and fault regression tests.
use ikria_harness::{
    flight::{ABORT, FlightProgram, HOP},
    session::{self, Mode},
};
use ikria_records::flight::{self, BridgeFrame, Commanded, HarnessFrame, Telemetry};
use ikria_records::{GameTime, PartId, Situation, Vector3, VesselId};
use keleusma::Arena;
use std::io::Cursor;
fn sample() -> Telemetry {
    Telemetry {
        id: VesselId(1),
        situation: Situation::PreLaunch,
        altitude_m: 8.0,
        vertical_speed_m_s: 0.0,
        solid_fuel: 35.0,
        mass_kg: 1688.5,
        crew: 0,
        controllable: false,
        has_link: false,
        physics_active: true,
        parts: 8,
        engine_active: false,
        chute_deployed: false,
        chute_safe: true,
        up_body: Vector3::new(0.0, 1.0, 0.0).unwrap(),
        rates_body: Vector3::new(0.0, 0.0, 0.0).unwrap(),
    }
}
fn wire(frames: &[BridgeFrame]) -> Cursor<Vec<u8>> {
    let mut bytes = Vec::new();
    for frame in frames {
        flight::write_frame(&mut bytes, frame).unwrap();
    }
    Cursor::new(bytes)
}
fn hello(armed: bool) -> BridgeFrame {
    BridgeFrame::Hello {
        version: flight::VERSION,
        save: flight::VERIFICATION_SAVE.into(),
        vessel: "IKR-TS-001B".into(),
        armed,
        engine: PartId(1),
        chute: PartId(2),
    }
}
fn tick(seq: u64, t: Telemetry) -> BridgeFrame {
    BridgeFrame::Sample {
        seq,
        ut: GameTime::new(100.0 + seq as f64 * 0.02).unwrap(),
        telemetry: t,
    }
}
#[test]
fn hop_phase_sequence_and_one_shot_actions() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
    assert!(p.cycles() > 0);
    let mut t = sample();
    let first = p.step(&t, 0, false).unwrap();
    assert_eq!((first.phase, first.ignite), (1, 1));
    t.situation = Situation::Flying;
    t.engine_active = true;
    t.altitude_m = 50.0;
    assert_eq!(p.step(&t, 1, false).unwrap().ignite, 0);
    t.solid_fuel = 0.0;
    t.vertical_speed_m_s = 10.0;
    let coast = p.step(&t, 1, false).unwrap();
    assert_eq!((coast.phase, coast.deploy), (2, 0));
    t.vertical_speed_m_s = -2.0;
    let chute = p.step(&t, 2, false).unwrap();
    assert_eq!((chute.phase, chute.deploy), (3, 1));
    t.chute_deployed = true;
    assert_eq!(p.step(&t, 3, false).unwrap().deploy, 0);
    t.situation = Situation::Landed;
    assert_eq!(p.step(&t, 3, false).unwrap().phase, 4);
}
#[test]
fn abort_inhibits_pad_ignition_and_recovers_after_burnout() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
    let mut t = sample();
    let pad = p.step(&t, 0, true).unwrap();
    assert_eq!((pad.phase, pad.ignite), (4, 0));
    t.situation = Situation::Flying;
    t.vertical_speed_m_s = -2.0;
    t.altitude_m = 100.0;
    let burn = p.step(&t, 1, true).unwrap();
    assert_eq!((burn.phase, burn.deploy, burn.throttle), (5, 0, 0));
    t.solid_fuel = 0.0;
    let recovery = p.step(&t, 5, true).unwrap();
    assert_eq!(recovery.deploy, 1);
}
#[test]
fn deployment_boundaries_match_policy() {
    for (alt, speed, fuel, safe) in [
        (5.0, -2.0, 0.0, true),
        (1500.0, -2.0, 0.0, true),
        (100.0, -1.0, 0.0, true),
        (100.0, -2.0, 0.01, true),
        (100.0, -2.0, 0.0, false),
    ] {
        let arena = Arena::with_capacity(1024 * 1024);
        let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
        let mut t = sample();
        t.situation = Situation::Flying;
        t.altitude_m = alt;
        t.vertical_speed_m_s = speed;
        t.solid_fuel = fuel;
        t.chute_safe = safe;
        assert_eq!(p.step(&t, 2, false).unwrap().deploy, 0);
    }
}
#[test]
fn incompatible_dialogue_and_cycle_budget_are_rejected() {
    let arena = Arena::with_capacity(1024 * 1024);
    assert!(FlightProgram::new(HOP, &arena, 1).is_err());
    assert!(FlightProgram::new(&HOP.replace("a: FlightInput", "a: Word"), &arena, 10000).is_err());
    assert!(FlightProgram::new(&" ".repeat(65537), &arena, 10000).is_err());
}
#[test]
fn invalid_output_and_vm_fault_are_terminal() {
    for body in [
        HOP.replace("ignite: ignition", "ignite: 2"),
        HOP.replace("ignite: ignition", "ignite: 1 / a.phase"),
    ] {
        let arena = Arena::with_capacity(1024 * 1024);
        let mut p = FlightProgram::new(&body, &arena, 10000).unwrap();
        assert!(p.step(&sample(), 0, false).is_err());
        assert!(p.step(&sample(), 1, false).is_err());
    }
}
#[test]
fn nonfinite_telemetry_never_enters_vm() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
    let mut t = sample();
    t.mass_kg = f64::NAN;
    assert!(p.step(&t, 0, false).is_err());
}
#[test]
fn attitude_signs_and_saturation_are_explicit() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
    let mut t = sample();
    t.up_body = Vector3::new(0.6, 0.0, -0.8).unwrap();
    let out = p.step(&t, 0, false).unwrap();
    assert_eq!((out.pitch, out.yaw), (1000, 1000));
    t.up_body = Vector3::new(0.0, 1.0, 0.0).unwrap();
    t.rates_body = Vector3::new(-0.2, 0.2, 0.2).unwrap();
    let out = p.step(&t, 1, false).unwrap();
    assert_eq!((out.pitch, out.yaw, out.roll), (-400, -400, -400));
}
#[test]
fn chronology_and_authority_refusals() {
    let t = sample();
    let ut = GameTime::new(1.0).unwrap();
    assert!(session::validate_sample(&t, 1, 0, ut, None, None).is_err());
    assert!(session::validate_sample(&t, 0, 0, ut, Some(ut), None).is_err());
    assert!(session::validate_sample(&t, 0, 0, ut, None, Some(7)).is_err());
    for which in 0..3 {
        let mut changed = t.clone();
        match which {
            0 => changed.crew = 1,
            1 => changed.physics_active = false,
            _ => changed.id = VesselId(9),
        }
        assert!(session::validate_sample(&changed, 0, 0, ut, None, None).is_err());
    }
}
#[test]
fn monitoring_records_acknowledged_neutral_command() {
    let mut input = wire(&[
        hello(false),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ]);
    let mut output = Vec::new();
    let mut log = Vec::new();
    session::run(&mut input, &mut output, &mut log, Mode::Monitor, 1, HOP).unwrap();
    let mut reader = Cursor::new(output);
    let _: HarnessFrame = flight::read_frame(&mut reader).unwrap();
    let command: HarnessFrame = flight::read_frame(&mut reader).unwrap();
    assert!(matches!(command,HarnessFrame::Command{command,..} if command.requests.is_empty()));
    assert!(String::from_utf8(log).unwrap().contains("command_intent"));
}
#[test]
fn stale_sample_sends_no_command() {
    let mut input = wire(&[hello(false), tick(1, sample())]);
    let mut output = Vec::new();
    assert!(
        session::run(
            &mut input,
            &mut output,
            &mut Vec::new(),
            Mode::Monitor,
            2,
            HOP
        )
        .is_err()
    );
    assert_eq!(String::from_utf8(output).unwrap().lines().count(), 1);
}
#[test]
fn missing_ack_is_not_retried() {
    let mut input = wire(&[hello(true), tick(0, sample())]);
    let mut output = Vec::new();
    assert!(session::run(&mut input, &mut output, &mut Vec::new(), Mode::Hop, 2, HOP).is_err());
    assert_eq!(
        String::from_utf8(output)
            .unwrap()
            .matches("activate_engine")
            .count(),
        1
    );
}
#[test]
fn runtime_fault_switches_to_independent_abort_before_ignition() {
    let mut input = wire(&[
        hello(true),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ]);
    let mut output = Vec::new();
    let mut log = Vec::new();
    let fault = HOP.replace("ignite: ignition", "ignite: 1 / a.phase");
    session::run(&mut input, &mut output, &mut log, Mode::Hop, 1, &fault).unwrap();
    assert!(
        !String::from_utf8(output)
            .unwrap()
            .contains("activate_engine")
    );
    assert!(String::from_utf8(log).unwrap().contains("program_fault"));
}
struct BrokenLog;
impl std::io::Write for BrokenLog {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("disk full"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn log_failure_prevents_handshake_or_commands() {
    let mut input = wire(&[hello(true)]);
    let mut output = Vec::new();
    assert!(session::run(&mut input, &mut output, &mut BrokenLog, Mode::Hop, 1, HOP).is_err());
    assert!(output.is_empty());
}

#[test]
fn external_abort_latches_before_ignition() {
    let mut input = wire(&[
        hello(true),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ]);
    let mut output = Vec::new();
    let mut log = Vec::new();
    session::run_with_abort(&mut input, &mut output, &mut log, Mode::Hop, 1, HOP, || {
        true
    })
    .unwrap();
    assert!(
        !String::from_utf8(output)
            .unwrap()
            .contains("activate_engine")
    );
    assert!(String::from_utf8(log).unwrap().contains("abort_requested"));
}

#[test]
fn armed_session_cannot_begin_while_airborne() {
    let mut t = sample();
    t.situation = Situation::Flying;
    let mut input = wire(&[hello(true), tick(0, t)]);
    let mut output = Vec::new();
    assert!(session::run(&mut input, &mut output, &mut Vec::new(), Mode::Hop, 1, HOP).is_err());
    assert!(!String::from_utf8(output).unwrap().contains("command"));
}

#[test]
fn touchdown_bounce_returns_to_descent_without_new_discrete_action() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut p = FlightProgram::new(HOP, &arena, 10000).unwrap();
    let mut t = sample();
    t.solid_fuel = 0.0;
    t.chute_deployed = true;
    t.situation = Situation::Landed;
    assert_eq!(p.step(&t, 3, false).unwrap().phase, 4);
    t.situation = Situation::Flying;
    t.vertical_speed_m_s = 0.5;
    let bounce = p.step(&t, 4, false).unwrap();
    assert_eq!((bounce.phase, bounce.ignite, bounce.deploy), (3, 0, 0));
    t.situation = Situation::Landed;
    assert_eq!(p.step(&t, 3, false).unwrap().phase, 4);
}

#[test]
fn session_requires_a_settled_second_after_the_last_bounce() {
    let mut frames = vec![hello(true)];
    for seq in 0..60 {
        let mut t = sample();
        if seq > 0 {
            t.engine_active = true;
            t.solid_fuel = 0.0;
            t.situation = Situation::Flying;
            t.altitude_m = 100.0;
            t.vertical_speed_m_s = -3.0;
        }
        if seq >= 2 {
            t.chute_deployed = true;
        }
        if seq == 3 || seq >= 5 {
            t.situation = Situation::Landed;
            t.vertical_speed_m_s = 0.0;
            t.altitude_m = 2.0;
        }
        if seq == 4 {
            t.vertical_speed_m_s = 0.5;
            t.altitude_m = 2.1;
        }
        frames.push(tick(seq, t));
        frames.push(BridgeFrame::Applied { seq });
    }
    let mut input = wire(&frames);
    let mut output = Vec::new();
    let mut log = Vec::new();
    session::run(&mut input, &mut output, &mut log, Mode::Hop, 60, HOP).unwrap();
    let text = String::from_utf8(log).unwrap();
    let completed = text
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .find(|row| row["event"] == "complete")
        .unwrap();
    assert!(completed["ut"].as_f64().unwrap() >= 101.1);
    let output = String::from_utf8(output).unwrap();
    assert_eq!(output.matches("activate_engine").count(), 1);
    assert_eq!(output.matches("deploy_parachute").count(), 1);
}

#[test]
fn dialogue_ignores_source_location_but_not_type() {
    let arena = Arena::with_capacity(1024 * 1024);
    assert!(
        FlightProgram::new(
            &format!("// Additional program documentation\n{HOP}"),
            &arena,
            10000
        )
        .is_ok()
    );
}
fn run_frames(frames: &[BridgeFrame], mode: Mode, source: &str) -> (bool, String, String) {
    let mut input = wire(frames);
    let mut output = Vec::new();
    let mut log = Vec::new();
    let ticks = frames
        .iter()
        .filter(|f| matches!(f, BridgeFrame::Sample { .. }))
        .count();
    let result = session::run(
        &mut input,
        &mut output,
        &mut log,
        mode,
        ticks as u64,
        source,
    );
    (
        result.is_ok(),
        String::from_utf8(output).unwrap(),
        String::from_utf8(log).unwrap(),
    )
}
fn records(log: &str) -> Vec<serde_json::Value> {
    log.lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
/// A descent under the parachute that settles on the ground, appended from `seq`.
fn settle(frames: &mut Vec<BridgeFrame>, from: u64) {
    for seq in from..from + 60 {
        let mut t = sample();
        t.engine_active = true;
        t.solid_fuel = 0.0;
        t.chute_deployed = true;
        t.situation = Situation::Landed;
        t.altitude_m = 2.0;
        frames.push(tick(seq, t));
        frames.push(BridgeFrame::Applied { seq });
    }
}
#[test]
fn refused_normal_output_selects_the_abort_program() {
    // A brief Landed report during boost leads hop.kl into phase 4, and its next output
    // asks for boost again, a transition the session refuses. That refusal is a
    // program fault, so the abort program must take over instead of ending the session.
    let mut frames = vec![hello(true)];
    for seq in 0..6 {
        let mut t = sample();
        if seq > 0 {
            t.engine_active = true;
            t.situation = Situation::Flying;
            t.solid_fuel = 30.0 - seq as f64;
            t.altitude_m = 20.0;
            t.vertical_speed_m_s = 10.0;
        }
        if seq == 2 {
            t.situation = Situation::Landed;
            t.vertical_speed_m_s = 0.2;
        }
        if seq >= 4 {
            t.solid_fuel = 0.0;
            t.vertical_speed_m_s = -3.0;
            t.altitude_m = 100.0;
        }
        frames.push(tick(seq, t));
        frames.push(BridgeFrame::Applied { seq });
    }
    settle(&mut frames, 6);
    let (ok, output, log) = run_frames(&frames, Mode::Hop, HOP);
    assert!(ok, "session ended instead of aborting: {log}");
    let rows = records(&log);
    let fault = rows.iter().find(|r| r["event"] == "program_fault").unwrap();
    assert_eq!(fault["seq"], 3);
    let complete = rows.iter().find(|r| r["event"] == "complete").unwrap();
    assert_eq!(complete["aborted"], true);
    assert_eq!(output.matches("activate_engine").count(), 1);
    assert_eq!(output.matches("deploy_parachute").count(), 1);
}
#[test]
fn refused_abort_output_ends_the_session_without_sending_it() {
    let igniting = ABORT.replace("ignite: 0", "ignite: 1");
    let steering = ABORT.replace("pitch: 0,", "pitch: 500,");
    assert_ne!(igniting, ABORT);
    assert_ne!(steering, ABORT);
    let frames = [
        hello(true),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ];
    for (abort, ends) in [
        (ABORT, false),
        (igniting.as_str(), true),
        (steering.as_str(), true),
    ] {
        let mut output = Vec::new();
        let result = session::run_with_sources(
            &mut wire(&frames),
            &mut output,
            &mut Vec::new(),
            Mode::PadAbort,
            1,
            session::ProgramSources { normal: HOP, abort },
            || false,
        );
        assert_eq!(result.is_err(), ends);
        // A refused abort output sends no command at all, only the hello.
        let lines = String::from_utf8(output).unwrap().lines().count();
        assert_eq!(lines, if ends { 1 } else { 2 });
    }
}
#[test]
fn default_abort_program_is_not_the_normal_source() {
    let frames = [
        hello(false),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ];
    let (ok, _, log) = run_frames(&frames, Mode::Monitor, HOP);
    assert!(ok);
    let header = &records(&log)[0];
    assert_eq!(header["event"], "session");
    assert_ne!(header["program_sha256"], header["abort_sha256"]);
}
/// The validated domain's bounds and the deployment envelope's edges, in both
/// directions, so that rounding to thousandths is exercised, in every situation the
/// abort program can meet, with and without control authority.
fn boundary_samples() -> Vec<Telemetry> {
    let mut out = Vec::new();
    for altitude in [-1000.0, 5.0, 5.0006, 1499.9994, 1500.0, 1e9] {
        for speed in [-1e6, -1.0006, -1.0, 1e6] {
            for fuel in [0.0, 0.0094, 0.01, 1e9] {
                for situation in [
                    Situation::PreLaunch,
                    Situation::Flying,
                    Situation::Landed,
                    Situation::Splashed,
                ] {
                    for (safe, deployed) in [(true, false), (false, false), (true, true)] {
                        for controllable in [true, false] {
                            for rate in [-1000.0, 1000.0] {
                                let mut t = sample();
                                t.controllable = controllable;
                                t.situation = situation;
                                t.altitude_m = altitude;
                                t.vertical_speed_m_s = speed;
                                t.solid_fuel = fuel;
                                t.chute_safe = safe;
                                t.chute_deployed = deployed;
                                t.rates_body = Vector3::new(rate, -rate, rate).unwrap();
                                t.up_body = Vector3::new(0.0, 0.0, -1.0).unwrap();
                                out.push(t);
                            }
                        }
                    }
                }
            }
        }
    }
    out
}
#[test]
fn abort_program_is_total_and_inside_the_session_rules_over_the_validated_bounds() {
    let arena = Arena::with_capacity(1024 * 1024);
    let mut abort = FlightProgram::new(ABORT, &arena, 10000).unwrap();
    let profile = session::Profile {
        engine: PartId(1),
        chute: PartId(2),
        armed: true,
    };
    let (mut pad_aborts, mut completions, mut deployments) = (0, 0, 0);
    for t in &boundary_samples() {
        // Phase 0 is reachable only on the first sample, untouched on the pad, with
        // nothing commanded. Every later phase follows a commanded ignition.
        let mut cases = Vec::new();
        if t.situation == Situation::PreLaunch && !t.chute_deployed {
            cases.push((0, Commanded::default()));
        }
        for deployment in [false, true] {
            let commanded = Commanded {
                ignition: true,
                deployment,
            };
            cases.extend((1..=5).map(|phase| (phase, commanded)));
        }
        for (phase, commanded) in cases {
            let view = commanded.latch(t);
            let out = abort.step(&view, phase, true).unwrap();
            // The session's own decision, with the phase table, the completion guard,
            // the abort rules and the latched policy.
            let (next, command) =
                session::decide(&view, phase, true, out, &profile, commanded).unwrap();
            pad_aborts += i32::from(phase == 0 && next == 4);
            completions += i32::from(phase != 0 && next == 4);
            deployments += command.requests.len();
        }
    }
    // The grid reaches every kind of decision, so the checks above were not vacuous.
    assert!(pad_aborts > 0 && completions > 0 && deployments > 0);
}
#[test]
fn sample_that_faults_the_normal_program_does_not_fault_the_abort_program() {
    // A normal program whose attitude law divides by the roll rate faults on a sample
    // with no roll. Used as its own abort program, as the hop source formerly was, it
    // faults again and the session ends. The distinct default abort program reads no
    // attitude input, completes the pad abort and never ignites.
    let faulty = HOP.replace(
        "let aborting =",
        "let _roll_gain = 1000 / a.roll_rate;\n    let aborting =",
    );
    assert_ne!(faulty, HOP);
    let frames = [
        hello(true),
        tick(0, sample()),
        BridgeFrame::Applied { seq: 0 },
    ];
    let mut shared = Vec::new();
    let mut shared_log = Vec::new();
    let result = session::run_with_sources(
        &mut wire(&frames),
        &mut shared,
        &mut shared_log,
        Mode::Hop,
        1,
        session::ProgramSources {
            normal: &faulty,
            abort: &faulty,
        },
        || false,
    );
    assert!(result.is_err());
    assert!(
        String::from_utf8(shared_log)
            .unwrap()
            .contains("program_fault")
    );
    let (ok, output, log) = run_frames(&frames, Mode::Hop, &faulty);
    assert!(ok, "{log}");
    assert!(log.contains("program_fault"));
    assert!(!output.contains("activate_engine"));
}
#[test]
fn lagging_parachute_flag_cannot_cause_a_second_deployment() {
    let mut frames = vec![hello(true)];
    for seq in 0..12 {
        let mut t = sample();
        if seq > 0 {
            t.engine_active = true;
            t.situation = Situation::Flying;
            t.solid_fuel = 0.0;
            t.altitude_m = 100.0 - seq as f64;
            t.vertical_speed_m_s = if seq == 1 { 5.0 } else { -3.0 };
        }
        // The deployment is commanded at sequence 2, and the flag lags until 8.
        t.chute_deployed = seq >= 8;
        frames.push(tick(seq, t));
        frames.push(BridgeFrame::Applied { seq });
    }
    settle(&mut frames, 12);
    let (ok, output, log) = run_frames(&frames, Mode::Hop, HOP);
    assert!(ok, "{log}");
    assert!(!log.contains("program_fault"));
    assert_eq!(output.matches("deploy_parachute").count(), 1);
}
#[test]
fn refused_samples_and_frames_are_journalled_before_refusal() {
    let mut crewed = sample();
    crewed.crew = 1;
    let unexpected = BridgeFrame::Failed {
        reason: "bridge stopped".into(),
    };
    for frames in [
        vec![hello(true), tick(0, crewed)],
        vec![hello(true), tick(1, sample())],
        vec![hello(true), tick(0, sample()), unexpected.clone()],
    ] {
        let refused = frames.last().unwrap().clone();
        let (ok, _, log) = run_frames(&frames, Mode::Hop, HOP);
        assert!(!ok);
        let last = records(&log).pop().unwrap();
        assert_eq!(
            serde_json::from_value::<BridgeFrame>(last).unwrap(),
            refused
        );
    }
    // A JSON line that is not a bridge frame is journalled as received, then refused.
    let mut bytes = wire(&[hello(true)]).into_inner();
    bytes.extend_from_slice(b"{\"type\":\"sample\",\"seq\":0}\n");
    let mut log = Vec::new();
    assert!(
        session::run(
            &mut Cursor::new(bytes),
            &mut Vec::new(),
            &mut log,
            Mode::Hop,
            1,
            HOP
        )
        .is_err()
    );
    let last = records(&String::from_utf8(log).unwrap()).pop().unwrap();
    assert_eq!(last["event"], "unparsed_frame");
    assert_eq!(last["frame"]["seq"], 0);
}
#[test]
fn decision_refuses_completion_in_flight_and_skipped_phases() {
    let profile = session::Profile {
        engine: PartId(1),
        chute: PartId(2),
        armed: true,
    };
    let neutral = |phase| ikria_harness::flight::FlightOutput {
        phase,
        ignite: 0,
        deploy: 0,
        throttle: 0,
        pitch: 0,
        yaw: 0,
        roll: 0,
    };
    let mut t = sample();
    t.engine_active = true;
    t.solid_fuel = 0.0;
    let decide = |t: &Telemetry, from, to| {
        session::decide(t, from, false, neutral(to), &profile, Commanded::default()).is_ok()
    };
    // Completion is accepted on the ground and refused in the air, from the same phase.
    t.situation = Situation::Landed;
    assert!(decide(&t, 3, 4));
    t.situation = Situation::Flying;
    assert!(!decide(&t, 3, 4));
    // The same output is accepted or refused by the phase it follows.
    assert!(decide(&t, 2, 3));
    assert!(!decide(&t, 5, 3));
    assert!(!decide(&t, 0, 2));
}
#[test]
fn refused_or_unparsed_hello_is_journalled() {
    let wrong_vessel = BridgeFrame::Hello {
        version: flight::VERSION,
        save: flight::VERIFICATION_SAVE.into(),
        vessel: "another vessel".into(),
        armed: true,
        engine: PartId(1),
        chute: PartId(2),
    };
    for first in [wrong_vessel, hello(false), BridgeFrame::Applied { seq: 0 }] {
        let mut output = Vec::new();
        let mut log = Vec::new();
        let result = session::run(
            &mut wire(std::slice::from_ref(&first)),
            &mut output,
            &mut log,
            Mode::Hop,
            1,
            HOP,
        );
        assert!(result.is_err());
        assert!(output.is_empty());
        let last = records(&String::from_utf8(log).unwrap()).pop().unwrap();
        assert_eq!(last["event"], "hello_refused");
        assert_eq!(
            serde_json::from_value::<BridgeFrame>(last["hello"].clone()).unwrap(),
            first
        );
    }
    let mut log = Vec::new();
    let unparsed = b"{\"type\":\"hello\"}\n".to_vec();
    assert!(
        session::run(
            &mut Cursor::new(unparsed),
            &mut Vec::new(),
            &mut log,
            Mode::Hop,
            1,
            HOP
        )
        .is_err()
    );
    let last = records(&String::from_utf8(log).unwrap()).pop().unwrap();
    assert_eq!(last["event"], "unparsed_frame");
}
#[test]
fn unparsed_line_near_the_frame_bound_is_journalled_as_an_excerpt() {
    // The longest line the reader accepts, one byte below the bound with its newline.
    let fill = |template: &str| {
        let pad = flight::MAX_FRAME - 2 - template.len() + 2;
        template.replace("{}", &"x".repeat(pad))
    };
    for line in [
        fill(r#"{"type":"sample","pad":"{}"}"#),
        fill(r#"{"type":"{}"}"#),
    ] {
        assert_eq!(line.len() + 1, flight::MAX_FRAME - 1);
        let mut bytes = wire(&[hello(true)]).into_inner();
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'\n');
        let mut log = Vec::new();
        assert!(
            session::run(
                &mut Cursor::new(bytes),
                &mut Vec::new(),
                &mut log,
                Mode::Hop,
                1,
                HOP
            )
            .is_err()
        );
        let last = records(&String::from_utf8(log).unwrap()).pop().unwrap();
        assert_eq!(last["event"], "unparsed_frame");
        assert_eq!(last["length"], line.len());
        let excerpt = last["excerpt"].as_str().unwrap();
        assert!(excerpt.len() <= session::UNPARSED_EXCERPT);
        assert!(last["error"].as_str().unwrap().len() <= session::UNPARSED_EXCERPT);
    }
}
#[test]
fn abort_request_file_fails_closed() {
    let base = std::env::temp_dir().join(format!("ikria-abort-{}", std::process::id()));
    std::fs::create_dir_all(&base).unwrap();
    let present = base.join("request");
    std::fs::write(&present, b"").unwrap();
    assert!(!session::abort_file_requested(None));
    assert!(!session::abort_file_requested(Some(&base.join("absent"))));
    assert!(session::abort_file_requested(Some(&present)));
    // A path that cannot be checked, here because its parent is a file, is not proof
    // of absence, so it counts as a request.
    let unknowable = present.join("child");
    assert!(std::fs::metadata(&unknowable).is_err());
    assert!(session::abort_file_requested(Some(&unknowable)));
    std::fs::remove_dir_all(&base).unwrap();
}
#[test]
fn invalid_command_is_not_hidden_by_authority_masking() {
    let out = ikria_harness::flight::FlightOutput {
        phase: 1,
        ignite: 2,
        deploy: 0,
        throttle: 0,
        pitch: 9000,
        yaw: 0,
        roll: 0,
    };
    assert!(ikria_harness::flight::command(&sample(), out, PartId(1), PartId(2)).is_err());
}
