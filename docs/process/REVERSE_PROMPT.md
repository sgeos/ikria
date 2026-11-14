# Reverse Prompt

> **Navigation**: [Process](./README.md)

A state file, overwritten whole after each task. It reports what was done, what was
decided and by whom, what was verified and how, what was not verified, and what the
operator needs to decide. Facts, inferences and hypotheses are marked as such.

## Merge authority and bridge licence, 2026-11-14

**Done.**
- Recorded in `CLAUDE.md` and `AGENTS.md` the operator's standing authority to merge
  pull requests in this repository and `sgeos/ikria-bridge`, until further notice.
- Recorded in the bridge repository that the operator confirmed GPL-3.0-only, and the
  same merge authority.

**Decided.** By the operator on 2026-11-14, the merge authority and the bridge's
licence.

**Verified.** `./scripts/check.sh` exits zero on this branch, on macOS.

**Not verified.** Nothing new.

**For the operator.** The push authority still leaves out code, so a code pull request
needs the operator's word before it is pushed. Whether to extend it is the operator's
call.
