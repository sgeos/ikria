# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by a Claude line on 2026-11-18.

## An hour of work that needed no operator attention

The operator asked on 2026-11-18 for obvious tasks that need no operator attention.
Two agents worked in separate worktrees, and two independent reviews checked their
code before merging.

**Done.**
- **Known flight-control defects fixed.** All seven, with tests that fail without each
  fix, merged in this branch.
  - Refused program outputs now select a distinct abort program, `programs/abort.kl`.
  - Deployment is latched.
  - Frames and sessions have wall-clock deadlines.
  - Every frame is journalled before validation.
  - The Python guards are tested so that deleting any one fails a test, 41 of 42 by
    mutation.
  - Landings need a settle period.
- **R20 in the tools.** `tools/phase0/launch_site.py` recovers crewed vessels near the
  pad or runway before any launch, through kRPC in the flight scene and through a
  temporary save elsewhere, and `tools/sepratron/flightlib.py` uses it. Its save-file
  path found a real crewed plane at the runway's start in an old save.
- **T8, the save reader,** merged as `crates/ikria-save`, pull request 43. A review found
  that finished contracts were read from the wrong node, which the old Rookie save
  confirmed, and that is fixed.
- **Template sync.** The gate and worktree scripts of Ikria and of the bridge now match
  template commit `8a79622`, pull requests 42 here and 8 in the bridge. Both now scan
  binary files.

**Verified.** The gate exits zero on macOS with every file staged, 78 Rust tests in this
branch before the save reader's 42, 50 phase 0 Python tests and the gate's own 12. The
save reader and the launch check were run read-only on real non-campaign saves.

**Not verified.** No change was exercised in the game. The flight fixes still owe a live
check before a career flight.

**Closed after the report.** The operator loaded "ipv-jeb-restored", and the verification
save's roster lists Jebediah as Available. The operator said on 2026-11-18 that exact
wording in the documents is not a concern, so the template's wording for concurrent lines
and for the worktree copy of local material was adopted in `CLAUDE.md`. Ikria's
`AGENTS.md` keeps its Grok Build sentence.

**For the operator.** Nothing.
