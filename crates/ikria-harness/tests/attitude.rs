//! Tests of the representative attitude loop of Phase 0 experiment 8.
//!
//! Each test compares the Keleusma loop with the Rust reference, rather than with fixed
//! numbers, so that both sides of every comparison are derived.

use ikria_harness::attitude::{
    Attitude, AttitudeLoop, Command, KD, KP, LIMIT, LoopError, SOURCE, reference_command,
};
use keleusma::Arena;
use keleusma::vm::DEFAULT_ARENA_CAPACITY;

fn grid() -> Vec<Attitude> {
    let values = [-400, -150, -1, 0, 1, 37, 150, 400];
    let mut out = Vec::new();
    for &e in &values {
        for &r in &values {
            out.push(Attitude {
                pitch_err: e,
                yaw_err: -e,
                roll_err: e / 2,
                pitch_rate: r,
                yaw_rate: r / 3,
                roll_rate: -r,
            });
        }
    }
    out
}

#[test]
fn builds_and_verifies_with_a_bound() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let lp = AttitudeLoop::new(&arena).expect("the loop builds and verifies");
    assert!(lp.wcet_per_iteration().expect("a bound") > 0);
}

#[test]
fn every_step_matches_the_reference() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let mut lp = AttitudeLoop::new(&arena).expect("loop");
    // Many consecutive steps, so that the first call and the resume after each reset
    // are both exercised.
    for input in grid() {
        let got = lp.step(&input).expect("step");
        assert_eq!(got, reference_command(&input), "input {input:?}");
    }
}

#[test]
fn commands_clamp_at_the_limit() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let mut lp = AttitudeLoop::new(&arena).expect("loop");
    let far = Attitude {
        pitch_err: 1_000_000,
        yaw_err: -1_000_000,
        ..Attitude::default()
    };
    let got = lp.step(&far).expect("step");
    assert_eq!(
        got,
        Command {
            pitch: LIMIT,
            yaw: -LIMIT,
            roll: 0
        }
    );
    assert_eq!(got, reference_command(&far));
}

#[test]
fn source_and_constants_agree() {
    assert!(SOURCE.contains(&format!("err * {KP}")), "proportional gain");
    assert!(SOURCE.contains(&format!("rate * {KD}")), "derivative gain");
    assert!(
        SOURCE.contains(&format!("if x > {LIMIT} {{ {LIMIT} }}")),
        "limit"
    );
}

#[test]
fn a_loop_that_never_yields_is_refused() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let source = r#"
struct Attitude {
    pitch_err: Word, yaw_err: Word, roll_err: Word,
    pitch_rate: Word, yaw_rate: Word, roll_rate: Word
}
struct Command { pitch: Word, yaw: Word, roll: Word }
loop main(a: Attitude) -> Command {
    Command { pitch: a.pitch_err, yaw: 0, roll: 0 }
}
"#;
    match AttitudeLoop::from_source(source, &arena) {
        Err(LoopError::Build(_) | LoopError::Vm(_)) => {}
        Err(other) => panic!("refused for the wrong reason: {other}"),
        Ok(_) => panic!("a loop without a yield was accepted"),
    }
}

#[test]
fn source_that_does_not_parse_is_a_build_error() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    match AttitudeLoop::from_source("loop main(a: Attitude -> Command {", &arena) {
        Err(LoopError::Build(_)) => {}
        Err(other) => panic!("wrong error: {other}"),
        Ok(_) => panic!("malformed source was accepted"),
    }
}
