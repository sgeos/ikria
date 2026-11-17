//! Flight boundary, ordering and fault regression tests.
use ikria_records::{flight::*, *};
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
#[test]
fn bounded_frames_reject_truncation_oversize_and_unknown_fields() {
    for bytes in [
        Vec::new(),
        b"{\"type\":\"close\"}".to_vec(),
        vec![b' '; MAX_FRAME],
        b"{\"type\":\"close\",\"extra\":true}\n".to_vec(),
    ] {
        assert!(read_frame::<_, HarnessFrame>(&mut Cursor::new(bytes)).is_err());
    }
}
#[test]
fn framing_consumes_exactly_one_message() {
    let mut bytes = Vec::new();
    write_frame(&mut bytes, &HarnessFrame::Close {}).unwrap();
    write_frame(&mut bytes, &HarnessFrame::Hello { version: VERSION }).unwrap();
    let mut reader = Cursor::new(bytes);
    assert_eq!(
        read_frame::<_, HarnessFrame>(&mut reader).unwrap(),
        HarnessFrame::Close {}
    );
    assert_eq!(
        read_frame::<_, HarnessFrame>(&mut reader).unwrap(),
        HarnessFrame::Hello { version: VERSION }
    );
}
#[test]
fn telemetry_scalars_and_geometry_are_validated() {
    let mut t = sample();
    assert!(t.validate().is_ok());
    for mass in [-1.0, 0.0, f64::NAN, f64::INFINITY] {
        t.mass_kg = mass;
        assert!(t.validate().is_err());
    }
    t = sample();
    t.solid_fuel = -1.0;
    assert!(t.validate().is_err());
    t = sample();
    t.up_body = Vector3::new(0.0, 0.0, 0.0).unwrap();
    assert!(t.validate().is_err());
    t = sample();
    t.parts = 0;
    assert!(t.validate().is_err());
}
#[test]
fn ignition_requires_whole_profile_authorization() {
    let t = sample();
    let c = VesselCommand {
        id: t.id,
        requests: vec![Request::ActivateEngine { part: PartId(1) }],
        ..Default::default()
    };
    assert!(authorize(&t, &c, PartId(1), PartId(2), true).is_ok());
    assert!(authorize(&t, &c, PartId(1), PartId(2), false).is_err());
    for which in 0..6 {
        let mut changed = t.clone();
        match which {
            0 => changed.crew = 1,
            1 => changed.engine_active = true,
            2 => changed.chute_deployed = true,
            3 => changed.physics_active = false,
            4 => changed.situation = Situation::Flying,
            _ => changed.solid_fuel = 0.0,
        };
        assert!(authorize(&changed, &c, PartId(1), PartId(2), true).is_err());
    }
}
#[test]
fn unsupported_duplicate_and_wrong_part_requests_are_rejected() {
    for requests in [
        vec![Request::Abort],
        vec![Request::ActivateEngine { part: PartId(9) }],
        vec![Request::ActivateEngine { part: PartId(1) }; 2],
    ] {
        let c = VesselCommand {
            id: VesselId(1),
            requests,
            ..Default::default()
        };
        assert!(authorize(&sample(), &c, PartId(1), PartId(2), true).is_err());
    }
}
#[test]
fn uncontrollable_vessel_cannot_receive_nonzero_axes() {
    let c = VesselCommand {
        id: VesselId(1),
        pitch: Axis::new(0.1).unwrap(),
        ..Default::default()
    };
    assert!(authorize(&sample(), &c, PartId(1), PartId(2), true).is_err());
}
#[test]
fn parachute_requires_burnout_descent_safe_height_and_correct_part() {
    let mut t = sample();
    t.situation = Situation::Flying;
    t.solid_fuel = 0.0;
    t.vertical_speed_m_s = -3.0;
    t.altitude_m = 100.0;
    let c = VesselCommand {
        id: t.id,
        requests: vec![Request::DeployParachute { part: PartId(2) }],
        ..Default::default()
    };
    assert!(authorize(&t, &c, PartId(1), PartId(2), true).is_ok());
    for which in 0..6 {
        let mut x = t.clone();
        match which {
            0 => x.chute_safe = false,
            1 => x.chute_deployed = true,
            2 => x.solid_fuel = 0.01,
            3 => x.altitude_m = 5.0,
            4 => x.altitude_m = 1500.0,
            _ => x.vertical_speed_m_s = -1.0,
        };
        assert!(authorize(&x, &c, PartId(1), PartId(2), true).is_err());
    }
}

#[test]
fn game_time_roundtrips_bit_exactly_at_observed_epoch() {
    for offset in 0..10000 {
        let time = 187634.1029476383_f64 + f64::from(offset) * 0.02;
        let original = HarnessFrame::Command {
            seq: offset as u64,
            ut: GameTime::new(time).unwrap(),
            command: VesselCommand::default(),
        };
        let mut bytes = Vec::new();
        write_frame(&mut bytes, &original).unwrap();
        let decoded: HarnessFrame = read_frame(&mut Cursor::new(bytes)).unwrap();
        assert_eq!(original, decoded);
    }
}
