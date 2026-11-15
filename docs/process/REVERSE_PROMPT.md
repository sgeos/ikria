# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## The test career and the cheat bar, 2026-11-14

**Done.**
- Confirmed the operator's new save "Ikria Physical Verification". It is a career on
  Hard, with quickload, revert and return to the editor enabled, which the game marks
  Custom. Task T13 of [`V0_1_0.md`](../roadmap/V0_1_0.md) is done, and the monolith
  test is task T14.
- Found that kRPC ships a `KRPC.Debug` service that acts as a cheat menu. The harness
  design now bars it from the campaign's career and its sandbox qualification runs, and
  allows it only in the test save for setting up tests.

**Decided.** Nothing new.

**Verified.**
- The save's settings were compared line by line with the campaign career's.
- The save file holds no monolith positions.
- kRPC has no KerbNet or anomaly interface.
- `./scripts/check.sh` exits zero on this branch with every file staged, on macOS.

**Not verified.** Anything about the monoliths themselves, which task T14 tests.

**For the operator.** Whether the `Debug` service may be used in the test save to set up
the monolith test, and the operator's part in it, which needs the game's interface.
