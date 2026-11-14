# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Design decisions, 2026-11-14

**Done.**
- Recorded the operator's decisions in
  [`docs/architecture/HARNESS_DESIGN.md`](../architecture/HARNESS_DESIGN.md). The
  layout is three processes, and the bridge lives in a separate GPL-3.0 repository
  using `krpc-client`.
- Recorded the ruling on O9 in the campaign plan's fifth set of rulings, and amended
  C16 to match.

**Decided.** By the operator on 2026-11-14, D1, D2 and O9.

**Verified.** `./scripts/check.sh` exits zero on this branch, on macOS. The kRPC server
was still not reachable when checked, so no experiment has run.

**Not verified.** Everything the Phase 0 experiments are to measure.

**For the operator.**
- Who creates the bridge repository, and whether it is public like this one. This line
  can create it from `sgeos/project-template` on the operator's word, or the admin line
  can, as it seeded this repository.
- Loading the sandbox save, so that the experiments can run.
