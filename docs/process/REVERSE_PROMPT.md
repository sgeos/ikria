# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Seed review, 2026-11-13

From the receiving line, its first session.

**Done.**
- Ran the handoff's validity checks. The handoff was valid.
- Reviewed every tracked file, every commit message and diff, and the repository text on
  GitHub, and reported to the operator in session.
- Recorded the operator's rulings that belong in the record. Charter item 10 states the
  goal and allows normal KSP terminology. The charter's context records the operator's
  note on the costs of a simulation.
- Restated the rationale for keeping KSP types behind the bridge in the handoff's next
  step, as this line's recommendation.
- Brought `CLAUDE.md` and `AGENTS.md` closer to the template's wording.

**Decided.** By the operator, in session on 2026-11-13, as recorded above.

**Verified.** `./scripts/check.sh` and `./scripts/check.sh --history --hygiene-only` both
exit zero on this branch, on macOS. Continuous integration had not run when this was
written. Its result is on the pull request.

**Not verified.** The charter's item 10 keeps the operator's words "vehicle-control
routine pairs". This line reads them as pairs of a craft and its control routine, which
is an inference and not confirmed.

**For the operator.** Confirm or correct the reading of item 10.
