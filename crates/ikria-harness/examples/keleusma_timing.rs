//! Phase 0 experiment 8. Measures the time per step of the representative attitude loop
//! on this machine, and prints the verifier's bound for comparison.
//!
//! Run with `cargo run --release -p ikria-harness --example keleusma_timing`. The figures
//! hold for the machine and build that produced them, and the release profile is the one
//! that matters for the harness.

use std::time::Instant;

use ikria_harness::attitude::{Attitude, AttitudeLoop, reference_command};
use keleusma::Arena;
use keleusma::vm::DEFAULT_ARENA_CAPACITY;

const WARMUP: usize = 10_000;
const STEPS: usize = 200_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let mut lp = AttitudeLoop::new(&arena)?;
    let bound = lp.wcet_per_iteration()?;

    let input = |i: usize| {
        let x = i64::try_from(i % 2000).unwrap_or(0) - 1000;
        Attitude {
            pitch_err: x,
            yaw_err: -x,
            roll_err: x / 2,
            pitch_rate: x / 3,
            yaw_rate: 0,
            roll_rate: -x / 4,
        }
    };
    for i in 0..WARMUP {
        lp.step(&input(i))?;
    }

    let mut samples = Vec::with_capacity(STEPS);
    let mut mismatches = 0usize;
    for i in 0..STEPS {
        let a = input(i);
        let t0 = Instant::now();
        let c = lp.step(&a)?;
        samples.push(t0.elapsed().as_nanos());
        if c != reference_command(&a) {
            mismatches += 1;
        }
    }
    samples.sort_unstable();
    let pick = |q: f64| {
        let idx = (q * (STEPS - 1) as f64).round() as usize;
        samples[idx.min(STEPS - 1)]
    };
    let mean = samples.iter().sum::<u128>() / STEPS as u128;
    println!("steps measured: {STEPS} after {WARMUP} warm-up steps");
    println!("mismatches against the Rust reference: {mismatches}");
    println!("verifier bound per iteration: {bound} nominal cycles");
    println!(
        "time per step, ns: median {}, mean {mean}, p99 {}, p99.9 {}, max {}",
        pick(0.5),
        pick(0.99),
        pick(0.999),
        samples[STEPS - 1]
    );
    let tick_ns = 20_000_000u128;
    println!(
        "steps that fit one 20 ms tick at the p99.9 time: {}",
        tick_ns / pick(0.999).max(1)
    );
    Ok(())
}
