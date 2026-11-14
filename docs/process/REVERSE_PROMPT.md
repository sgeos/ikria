# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Disposal policy and Phase 0 results, 2026-11-14

**Done.**
- Recorded the operator's debris policy as rule R13 of the campaign plan. Every stage is
  recovered, deorbited, given a second purpose such as a relay or a surface base, or as
  a last resort logged with its orbit and state before deletion.
- Added the operator's design principles for landers and stages to the campaign
  sequence. They cover small solid motors paired with reaction control and small
  throttleable engines, landers left behind on the strength of an analysis of return on
  investment, and stages that serve as relays.
- Ran Phase 0 experiments 3 and 4 in the sandbox. A held lockstep cycle of 15 calls fits
  the game's 20 ms tick for one vessel. A kerbal outside the vessel took an
  extravehicular report and a surface sample through kRPC, and kept both on boarding.
- Updated R12 to the standing authority as it now stands.
- Reworded one KSP word in `VTHL_PROGRAM.md` that the terms scan had missed, because
  the gate ran before the new file was staged.

**Decided.** By the operator on 2026-11-14, the disposal policy and the design
principles.

**Verified.**
- The Hard multiplier scales each subject's cap. An extravehicular report from the pad
  had a cap of 1.44, which is 8 times 0.3 times 0.6.
- The small motors and engines, and their nodes, were read from the game's data files.
- `./scripts/check.sh` exits zero on this branch with every file staged, on macOS.

**Not verified.**
- Planting a flag. The kerbal's `PlantFlag` event was inactive on the pad and planted
  nothing there.
- Whether kRPC can delete a vessel, which R13's last resort needs.
- The physics range of a separated stage, and several vessels at once.

**For the operator.** The terms scan of the history now fails on that word in two
published commits. The local note on the terms list has the details and a
recommendation.
