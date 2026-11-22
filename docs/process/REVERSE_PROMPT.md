# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by a Claude line on 2026-11-21.

## Handoff to Codex

**Done.** At the operator's request after the machine restart, `HANDOFF.md` is rewritten
for a receiving Codex line. It states what has merged since Codex's last handoff, the
rulings of 2026-11-18, the precautions for game runs that the Sepratron program and
its incidents taught, and the open work in recommended order. The first item is a live
check of the flight-control fixes in the verification save.

**Checked before writing.** Both checkouts were clean and in step with origin. Ikria's
main was `c44c8c6`, and the bridge's was `ef85dd8`. Codex's four branches are all in
main. One new item appeared, Dependabot pull request 47, which bumps `sha2` and is left
for review in the open work.

**Verified.** The gate passed on main on macOS with 157 Rust tests, 50 Phase 0 Python
tests and the gate's own 12, none ignored.

**Not verified.** The game state after the restart. The handoff records the last state
observed on 2026-11-18.

**For the operator.** Nothing. Start the Codex line in the main checkout.
