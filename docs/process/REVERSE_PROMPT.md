# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Deferred rescue and solvency, 2026-11-14

**Done.** Recorded the operator's notes as the campaign plan's eleventh set of rulings.
- R5 now allows a deferred rescue, since kerbals need no life support, provided the
  stranded crew are left in a stable state.
- R18 adds a solvency rule. A reserve covers the failure penalties of the contracts in
  progress and a rescue, and purchases, launches and contracts respect it.
- The campaign sequence describes the reserve, and a duplicated heading there is
  removed.

**Decided.** By the operator on 2026-11-14, that a rescue may wait and that insolvency is
a realistic failure mode. The reserve's details are proposed.

**A process defect.** The first commit of this branch carried only the campaign plan's
changes, because the editing script stopped at the duplicated heading while the gate and
the commit still ran. Its message claimed the sequence change as well. A second commit
completes the change. *Inference.* The editing step and the commit need the same
exit-status chaining as the gate.

**Verified.** Hard's negative funds and doubled penalties come from the career save's
settings. The gate exits zero on this branch with every file staged, on macOS, and
continuous integration passed on the first commit.

**Not verified.** That kerbals live indefinitely without life support, and can cross a
body on foot. Both are recalled, and the verification save can confirm them.

**For the operator.** Nothing new. The verification save still waits on a dialog being
closed, as `HANDOFF.md` records.
