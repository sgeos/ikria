# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Agency record, 2026-11-14

**Done.**
- Wrote [`docs/architecture/AGENCY_RECORD.md`](../architecture/AGENCY_RECORD.md) from
  the operator's request of 2026-11-14. It defines fifteen kinds of record, the
  provisional sections of the vehicle specification sheet and the mission plan and
  report, a reference table of differences between the game and the real world, a
  designation scheme and a directory layout.
- Recorded the request as the campaign plan's fourth set of rulings, which extends R8.
- Asked the blog line for articles on vehicle specification sheets and on mission
  planning and reporting, and suggested a third on program and financial management.
- Condensed the handoff. Its note on mastery checklists is dropped, since the campaign
  plan's criteria now cover it.

**Decided.** By the operator, the four requests and the two blog articles. The rest of
the record is proposed.

**Verified.** The message reached the blog session's inbox. `./scripts/check.sh` exits
zero on this branch, on macOS.

**Not verified.** The reference values for bodies and physics are recalled and marked
for checking. Whether kRPC exposes the reasons for changes in funds is open.

**For the operator.** Whether to take up the suggested article on program and financial
management. Otherwise nothing is blocking.
