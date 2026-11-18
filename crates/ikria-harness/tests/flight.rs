//! Flight boundary, ordering and fault regression tests.
use ikria_harness::{
    flight::{FlightProgram, HOP},
    session::{self, Mode},
};
use ikria_records::flight::{self, BridgeFrame, HarnessFrame, Telemetry};
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
