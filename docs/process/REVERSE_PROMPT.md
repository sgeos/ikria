# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by a Claude line on 2026-11-18, on receiving the Codex handoff.

## Receipt of the Codex handoff

**The handoff is valid.** The checkout was clean at `a4acb44`, whose commit describes the
state. The gate passed on macOS with 60 Rust and 13 Python tests, none ignored, which
matches the handoff's count. The disclosure scan over all 90 commits found nothing in
text files.

**Review of the Codex commits.** The commits from `614794e` to `a4acb44` were reviewed
for disclosure and for the operator's rulings, and an independent review read the
code.
- **Defects found.** The code review reported seven defects, now listed under "Known
  defects" in `FLIGHT_CONTROL.md`. The two most serious were confirmed against the code.
  An invalid program output, such as a refused phase transition, ends the session
  without the abort program. The abort program is the normal program's own source.
  I-0002 was a live instance of the first. A bounce made the program claim completion
  in flight, the completion guard refused it, and the session ended, while the article
  settled intact by chance. Its correction changed the program's logic, and a refused
  output still ends the session.
- **Tools pass.** The review's runs agree with the gate, with 60 Rust tests passing,
  clippy clean and 13 Python tests passing. Its mutation test showed that some Python
  boundary tests cannot fail on the guards they name.
- The eight compressed telemetry journals, about 18 MB decompressed, hold no listed
  term.
- The vehicle sheet makes no claim about real-world buildability, as the rulings
  require.
- The handoff rewrite dropped the origin of the project's name, which existed nowhere
  else. The README now carries it, and its stale status is corrected.

## Binary files in the disclosure scan

The gate's text scan uses `git grep -I`, which skips binary files, so binary and
compressed content was never scanned. A compiled Python file, committed by a Claude line
on 2026-11-14, was tracked by mistake. This branch makes the following changes.
- **Untracked.** The file is no longer tracked, and `.gitignore` excludes Python's
  compiled files.
- **A binary scan.** `scripts/scan_binary_terms.py` scans binary and gzip-compressed
  content, printing only paths, and the gate runs it. Its nine tests run in the gate.
- **The history scan catches it.** `./scripts/check.sh --history` now fails on the two
  versions of the file in the published history, as it should.

**Verified.** On macOS, the scanner's tests pass and the gate exits zero on this branch
with every file staged. The bridge repository holds no binary files.

**For the operator.**
1. A history decision described in the local notes, which would need a force-push
   and would invalidate every clone and worktree, including Codex's four.
2. Whether standing push and merge authority extends to Codex or Grok Build.
3. Whether to upstream the gate and worktree changes to the template.
