# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Booster program and science policy, 2026-11-14

**Done.**
- Wrote [`docs/architecture/VTHL_PROGRAM.md`](../architecture/VTHL_PROGRAM.md) for the
  operator's top-priority booster that takes off vertically and lands on the runway. It
  holds the requirement, the reasons it pays, the concept of operations with both return
  modes, the parts within reach, the design drivers, the risks, the control routines,
  and five development steps.
- Applied the operator's rulings on return on investment and on science before cash to
  [`CAMPAIGN_SEQUENCE.md`](../architecture/CAMPAIGN_SEQUENCE.md). The early Patents
  Licensing income is removed, and the funds gap rises to about 1.2 million. Development
  steps V1 to V3 of the booster join stage E2.
- Recorded the rulings as the campaign plan's sixth set, and added two experiments to
  the design's Phase 0 table.

**Decided.** By the operator on 2026-11-14, the program's return-on-investment basis,
science before cash, and the booster's priority.

**Verified.** Every part a first booster needs, and their costs, were read from the
game's data files. All of them lie inside the first research window. `./scripts/check.sh`
exits zero on this branch, on macOS.

**Not verified.**
- Whether the game removes a separated booster flying in the lower atmosphere. That is
  the program's main risk, and Phase 0 experiment 9 tests it.
- The return-on-investment figures of the booster, until boosters fly.

**For the operator.** Nothing is blocking. The sequence's assessment of the operator's
expectation for after the tree is there to be checked. It agrees, with funds as the
exception.
