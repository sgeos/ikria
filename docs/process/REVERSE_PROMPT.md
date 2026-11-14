# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Bridge repository and first Phase 0 results, 2026-11-14

**Done.**
- Created and seeded `sgeos/ikria-bridge` at the operator's instruction. It was made
  from `sgeos/project-template` v1.0.8 and licensed GPL-3.0-only.
  - Worked through the template's checklist, recorded the scope decision, and wrote the
    first handoff.
  - Applied the repository settings, made it public after the scans, and activated the
    default-branch ruleset.
- Ran Phase 0 experiments 1, 2 and 7 in part, in the sandbox save only, and recorded
  the results in [`HARNESS_DESIGN.md`](../architecture/HARNESS_DESIGN.md).

**Decided.** By the operator on 2026-11-14, the bridge repository, its visibility and
its seeding by this line.

**Verified.**
- The bridge's gate passed locally, and its CI passed on the seed commit.
- The terms scan covered the bridge's whole history, and a pattern search found no
  credentials in the seed.
- The kRPC connection, the tick rate of 50 per second, the tick hold, and the list of
  17 bodies and 145 biomes were read from the running game.
- `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.**
- No credential scanner such as `gitleaks` was run, since none is installed. The
  bridge's history holds only the template's commit and the seed.
- The experiments that need a vessel in flight have not run.

**For the operator.**
- Whether the bridge should be GPL-3.0-only, as seeded, or GPL-3.0-or-later.
- Dependabot's pull request 1 in the bridge, the same update Ikria merged, awaits a
  merge.
