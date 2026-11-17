//! Replay rejects changed commands and incomplete or mismatched evidence.
use ikria_harness::{
    flight::HOP,
    session::{self, Mode},
};
use ikria_records::{
    flight::{self, BridgeFrame, Telemetry},
    *,
};
use std::{
    fs,
    io::Cursor,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
fn journal() -> String {
    let t = Telemetry {
        id: VesselId(1),
        situation: Situation::PreLaunch,
        altitude_m: 2.0,
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
    };
    let mut input = Vec::new();
    for frame in [
        BridgeFrame::Hello {
            version: flight::VERSION,
            save: flight::VERIFICATION_SAVE.into(),
            vessel: "IKR-TS-001B".into(),
            armed: false,
            engine: PartId(1),
            chute: PartId(2),
        },
        BridgeFrame::Sample {
            seq: 0,
            ut: GameTime::new(100.0).unwrap(),
            telemetry: t,
        },
        BridgeFrame::Applied { seq: 0 },
    ] {
        flight::write_frame(&mut input, &frame).unwrap();
    }
    let mut log = Vec::new();
    session::run(
        &mut Cursor::new(input),
        &mut Vec::new(),
        &mut log,
        Mode::Monitor,
        1,
        HOP,
    )
    .unwrap();
    flight::write_frame(
        &mut log,
        &serde_json::json!({"event":"session_end","success":true}),
    )
    .unwrap();
    String::from_utf8(log).unwrap()
}
fn check(text: &str) -> bool {
    // Wall-clock resolution alone does not guarantee unique parallel test paths.
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let sequence = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tmp");
    fs::create_dir_all(&directory).unwrap();
    let name = format!(
        "replay-{}-{}-{}.jsonl",
        sequence,
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = directory.join(name);
    fs::write(&path, text).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_replay"))
        .arg(&path)
        .output()
        .unwrap();
    fs::remove_file(path).unwrap();
    output.status.success()
}
#[test]
fn recorded_command_replays() {
    assert!(check(&journal()));
}
#[test]
fn changed_command_is_detected() {
    assert!(!check(&journal().replace("\"pitch\":0.0", "\"pitch\":0.5")));
}
#[test]
fn failed_or_truncated_record_is_not_certified() {
    assert!(!check(
        &journal().replace("\"success\":true", "\"success\":false")
    ));
    let s = journal();
    assert!(!check(
        s.trim_end_matches('\n').rsplit_once('\n').unwrap().0
    ));
}
#[test]
fn different_program_is_rejected() {
    assert!(!check(
        &journal().replace("abort_sha256", "other_abort_sha256")
    ));
    assert!(!check(&journal().replace("program_sha256", "other_sha256")));
}
