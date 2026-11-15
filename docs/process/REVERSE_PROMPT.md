# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Green Monolith test, step 2 repeated, 2026-11-15

**Done.** Repeated step 2 of [`TP-0001`](../../record/verification/TP-0001.md) in the
verification save, after the operator reported that no dialog was open.
- The save identified itself as "Ikria Physical Verification", in career mode.
- The edited copy "ikria-edited" holds the start node and the rover's nine nodes, and
  the rover's thirteen part types all fall within them.
- kRPC's `load` hung again and was not applied, which refutes the open-dialog
  hypothesis.

**Decided.** Nothing new.

**Verified.** The facts above were read from the running game and the save files.

**Not verified.** Why `load` fails. *Inference.* A defect or limit of kRPC 0.7.0 at the
Space Center.

**For the operator.** Load "ikria-edited" from the game's Load menu. The line then
continues from step 3. The earlier report on deferred rescue and solvency is completed
and merged.
