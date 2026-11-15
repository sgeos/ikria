# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## First code, the workspace and Keleusma timing, 2026-11-14

**Done.**
- Created the Cargo workspace and the `ikria-harness` crate, which depends on Keleusma
  0.2.2 from crates.io. Added `cargo fmt`, `cargo clippy` and `cargo test` to the gate,
  the Rust toolchain to CI, and a `cargo` entry to Dependabot. That is task T6 of
  [`V0_1_0.md`](../roadmap/V0_1_0.md).
- Ran Phase 0 experiment 8, task T4. A representative three-axis attitude loop in
  Keleusma takes a median of 2.29 µs per step, and its verified bound is 67 nominal
  cycles.

**Decided.** Nothing new.

**Verified.**
- `RUSTC_WRAPPER= ./scripts/check.sh` exits zero on this branch with every file staged,
  on macOS. That covers formatting, clippy with warnings as errors, and the tests.
- The tests ran 6, and all 6 passed, with none skipped. They cover the loop against its
  Rust reference, clamping, a loop that never yields being refused, and a source that
  does not parse.
- The timing ran 200,000 steps with no mismatch against the reference.

**Not verified.**
- CI's first Rust run, until the pull request's checks report.
- The timing on any machine but this one.

**For the operator.** Nothing.
