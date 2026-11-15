# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Verification reports, 2026-11-14

**Done.**
- Began the record with [`record/verification/`](../../record/verification/README.md).
  Reports VR-0001 to VR-0007 document the Phase 0 experiments, each with its purpose,
  configuration, procedure, results, deviations and conclusions. The probe scripts are
  tracked in `tools/phase0/` and their raw outputs in `record/verification/data/`.
- Recorded the operator's rulings in
  [`AGENCY_RECORD.md`](../architecture/AGENCY_RECORD.md). R&D and verification are
  documented rehearsal, not cheating. Debug features are allowed in the verification
  save. Operations need as little of the operator's attention as possible, and the
  record is meant to be reusable.
- Deferred the Green Monolith test, task T14, because finding a monolith needs KerbNet's
  in-game window, and recorded why.

**Decided.** By the operator on 2026-11-14, as recorded above.

**Verified.**
- The reports restate results recorded at the time of each run.
- The tracked scripts and data hold no private paths.
- `RUSTC_WRAPPER= ./scripts/check.sh` exits zero on this branch with every file staged,
  on macOS.

**Not verified.**
- The one-vessel lockstep figures have no data file, as VR-0002 says.
- The scripts are their final versions, and the early runs that failed used drafts.

**For the operator.** Nothing.
