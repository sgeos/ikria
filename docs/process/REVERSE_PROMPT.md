# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Technology order, 2026-11-14

**Done.** Added a proposed technology order as the last appendix of
[`docs/architecture/CAMPAIGN_PLAN.md`](../architecture/CAMPAIGN_PLAN.md), answering the
operator's question on sequencing. It lists what the early nodes give and their
minimum cost in science, gives Kerbin's science per biome, estimates whether Hard has
enough science, and orders the tree in six steps.

**Decided.** Nothing new. The order is proposed.

**Verified.**
- The node contents, costs and prerequisites were read from the installed game's data
  files by a read-only script.
- Kerbin's six situation multipliers were read from the existing Normal career save.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.**
- The multipliers of bodies other than Kerbin are recalled, so the estimate that Hard
  has enough science is an inference. Phase 0 replaces it with an exact total.
- Whether the 60 percent reward on Hard scales each subject is recalled.
- That the first probe core has no stability assist and no reaction wheel is recalled.

**For the operator.** Whether the proposed order matches the operator's intent. It can
be merged as it stands, since it is marked proposed.
