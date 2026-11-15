# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Training, special sites and monolith preparation, 2026-11-14

**Done.**
- Wrote the astronaut training program,
  [`CREW_TRAINING.md`](../architecture/CREW_TRAINING.md), at the operator's request.
  It sets out recruitment, basic, lunar and interplanetary stages to level 5,
  procedures, and assignment. It also adds a proposed rule that no kerbal other than a
  pilot flies beyond the atmosphere at level 0, since such a kerbal cannot use the
  personal parachute.
- Widened C13 to every special site on the official wiki's list, and catalogued the
  sites with coordinates in `record/sites/`.
- Prepared the Green Monolith test on the operator's method, as test procedure
  `record/verification/TP-0001.md`. The guard, setup and research scripts are in
  `tools/phase0/`.

**Decided.** By the operator on 2026-11-14, the training program, the widened C13 and the
monolith method. The campaign plan records them as its eighth set of rulings.

**Verified.**
- The guard identified the loaded save correctly.
- Saving through kRPC worked in the verification save.
- `RUSTC_WRAPPER= ./scripts/check.sh` exits zero on this branch with every file staged,
  on macOS.

**Not verified.**
- The monolith test stopped at step 2. Launch, load and quickload through kRPC hung, and
  the game ended in its persistent state. A dialog left open by the first launch attempt
  is the likely cause, which is an inference.

**For the operator.**
- Look at the game window and close any open dialog, so that step 2 can be retried.
- Whether the toilet site, which needs a destroyed facility, stays out of C13.
