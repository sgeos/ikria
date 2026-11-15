//! Tests of the record types and the messages between processes, including every refusal.

use ikria_records::{
    Axis, BridgeMessage, Fraction, GameTime, HarnessMessage, PROTOCOL_VERSION, PartId, Quaternion,
    RecordError, Request, Resource, Situation, Vector3, VesselCommand, VesselId, VesselTelemetry,
    check_version, decode_bridge_message, decode_harness_message, encode,
};

fn telemetry() -> VesselTelemetry {
    VesselTelemetry {
        id: VesselId(7),
        situation: Situation::Flying,
        body: "Kerbin".into(),
        position: Vector3::new(600_000.0, 1.5, -2.25).unwrap(),
        velocity: Vector3::new(0.0, 120.5, 3.0).unwrap(),
        rotation: Quaternion::new([0.0, 0.0, 0.0, 1.0]).unwrap(),
        angular_velocity: Vector3::new(0.01, 0.0, -0.02).unwrap(),
        mass_kg: 1_840.0,
        available_thrust_n: 162_910.0,
        altitude_surface_m: 1_200.0,
        altitude_sea_level_m: 1_270.0,
        dynamic_pressure_pa: 8_000.0,
        stage: 1,
        has_link: true,
        resources: vec![Resource {
            name: "SolidFuel".into(),
            amount: 100.0,
            capacity: 205.0,
        }],
    }
}

fn command() -> VesselCommand {
    VesselCommand {
        id: VesselId(7),
        throttle: Fraction::new(0.75).unwrap(),
        pitch: Axis::new(-0.25).unwrap(),
        requests: vec![
            Request::ActionGroup { group: 3 },
            Request::ActivateEngine { part: PartId(42) },
        ],
        ..VesselCommand::default()
    }
}

#[test]
fn game_time_accepts_zero_and_refuses_bad_values() {
    assert_eq!(GameTime::new(0.0).unwrap().seconds(), 0.0);
    assert!(matches!(
        GameTime::new(-0.001),
        Err(RecordError::OutOfRange { .. })
    ));
    assert!(matches!(
        GameTime::new(f64::NAN),
        Err(RecordError::NotFinite(_))
    ));
    assert!(matches!(
        GameTime::new(f64::INFINITY),
        Err(RecordError::NotFinite(_))
    ));
}

#[test]
fn controls_accept_their_edges_and_refuse_beyond() {
    for v in [-1.0, 0.0, 1.0] {
        assert_eq!(Axis::new(v).unwrap().value(), v);
    }
    for v in [0.0, 1.0] {
        assert_eq!(Fraction::new(v).unwrap().value(), v);
    }
    assert!(Axis::new(1.000_001).is_err());
    assert!(Axis::new(-1.000_001).is_err());
    assert!(Fraction::new(-0.000_001).is_err());
    assert!(Fraction::new(1.000_001).is_err());
    assert!(Axis::new(f64::NAN).is_err());
}

#[test]
fn vectors_and_rotations_refuse_bad_values() {
    assert!(Vector3::new(0.0, f64::NAN, 0.0).is_err());
    assert!(Quaternion::new([0.0, 0.0, 0.0, 1.0]).is_ok());
    let half = 0.5_f64.sqrt();
    assert!(Quaternion::new([half, 0.0, 0.0, half]).is_ok());
    assert!(
        Quaternion::new([0.0, 0.0, 0.0, 1.01]).is_err(),
        "norm too large"
    );
    assert!(
        Quaternion::new([0.0, 0.0, 0.0, 0.0]).is_err(),
        "zero quaternion"
    );
    assert!(Quaternion::new([f64::INFINITY, 0.0, 0.0, 1.0]).is_err());
}

#[test]
fn a_tick_survives_a_round_trip() {
    let m = BridgeMessage::Tick {
        ut: GameTime::new(31_827.52).unwrap(),
        vessels: vec![telemetry()],
    };
    let line = encode(&m).unwrap();
    assert!(!line.contains('\n'), "one message per line");
    assert_eq!(decode_bridge_message(&line).unwrap(), m);
}

#[test]
fn commands_survive_a_round_trip() {
    let m = HarnessMessage::Commands {
        ut: GameTime::new(12.0).unwrap(),
        commands: vec![command()],
    };
    let line = encode(&m).unwrap();
    assert_eq!(decode_harness_message(&line).unwrap(), m);
}

#[test]
fn a_throttle_out_of_range_is_refused_on_decode() {
    let mut line = encode(&HarnessMessage::Commands {
        ut: GameTime::new(1.0).unwrap(),
        commands: vec![command()],
    })
    .unwrap();
    line = line.replace("\"throttle\":0.75", "\"throttle\":1.5");
    assert!(line.contains("1.5"), "the test edited the throttle");
    assert!(matches!(
        decode_harness_message(&line),
        Err(RecordError::Codec(_))
    ));
}

#[test]
fn a_negative_game_time_is_refused_on_decode() {
    let line = r#"{"type":"tick","ut":-1.0,"vessels":[]}"#;
    assert!(matches!(
        decode_bridge_message(line),
        Err(RecordError::Codec(_))
    ));
}

#[test]
fn an_action_group_outside_one_to_ten_is_refused() {
    for group in [0u8, 11] {
        let m = HarnessMessage::Commands {
            ut: GameTime::new(1.0).unwrap(),
            commands: vec![VesselCommand {
                requests: vec![Request::ActionGroup { group }],
                ..command()
            }],
        };
        let line = encode(&m).unwrap();
        assert!(
            matches!(
                decode_harness_message(&line),
                Err(RecordError::OutOfRange { .. })
            ),
            "group {group}"
        );
    }
}

#[test]
fn malformed_lines_are_codec_errors() {
    for line in [
        "",
        "not json",
        r#"{"type":"unknown"}"#,
        r#"{"type":"tick"}"#,
    ] {
        assert!(
            matches!(decode_bridge_message(line), Err(RecordError::Codec(_))),
            "line {line:?}"
        );
    }
}

#[test]
fn the_handshake_checks_the_version() {
    assert!(check_version(PROTOCOL_VERSION).is_ok());
    assert_eq!(
        check_version(PROTOCOL_VERSION + 1),
        Err(RecordError::VersionMismatch {
            ours: PROTOCOL_VERSION,
            theirs: PROTOCOL_VERSION + 1
        })
    );
    let hello = HarnessMessage::Hello {
        protocol_version: PROTOCOL_VERSION,
    };
    assert_eq!(
        decode_harness_message(&encode(&hello).unwrap()).unwrap(),
        hello
    );
}
