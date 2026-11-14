# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Phase 0 in the sandbox, 2026-11-14

**Done.** Finished the sandbox side of Phase 0, apart from experiment 9, and recorded
the results in [`HARNESS_DESIGN.md`](../architecture/HARNESS_DESIGN.md).
- **Experiment 4, the flag.** A kerbal that releases the ladder, stands clear and
  carries a flag can plant it through the `PlantFlag` event of its own module.
- **Experiment 10, two vessels.** A held cycle reading and commanding two vessels fits
  the 20 ms tick.
- **Experiment 7, inventory.** It counts about 3,100 science subjects, worth about
  218,000 science on Hard.
- **Experiment 9.** It moves to the booster's development step V3, since it needs a
  vehicle that holds its own attitude far from the active vessel.

**Decided.** Nothing new.

**Verified.** Each result above was observed in the sandbox save on 2026-11-14. The flag
was confirmed in the save file. `./scripts/check.sh` exits zero on this branch with
every file staged, on macOS.

**Not verified.**
- The subject count rests on stated assumptions and differs from the wiki's total by
  8 percent.
- Experiments 5 and 6 need the career save.

**For the operator.**
- Whether experiments 5 and 6 may run in the career save. They would roll a craft out to
  the pad and recover it at once, which refunds it in full.
- The sandbox has one planted flag, which kRPC cannot reach.
