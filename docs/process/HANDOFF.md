# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

Keep within 8 KB. History lives in git. Account for removed open items in commits.

## State on 2026-11-22

Written by Codex on `test/codex-flight-fix-live`, based on refreshed main
`7907697`, with bridge `ef85dd8`. Verification and publication are authorized.
The pull request records publication and required checks.

- **Milestone.** Every task of [`V0_1_0.md`](../roadmap/V0_1_0.md) is done except T12,
  partial, and T14, paused. Merged work follows.
  - **T8 and T9.** `crates/ikria-save` reads saves, and `crates/ikria-ledger` gives the
    science ledger, 3,104 subjects and 217,827 science on Hard. VR-0005 was corrected
    from 217,818.
  - **Flight-control defects fixed.** All seven, reviewed twice, in pull request 44. A
    refused output selects the distinct `programs/abort.kl`. See
    [FLIGHT_CONTROL.md](../architecture/FLIGHT_CONTROL.md). Partial live checks follow.
  - **VR-0012.** Monitor, default pad abort and refused-pad takeover passed and replayed
    exactly. Manual placement bypassed I-0008 once. A normal hop reached 1,241 m, then
    hit the bridge cycle deadline during parachute descent, I-0009. Passive landing
    and recovery retained eight parts. Normal completion and boost refusal remain open.
  - **T18, the Sepratron program.**
    [`SEPRATRON_PROGRAM.md`](../research/SEPRATRON_PROGRAM.md), TP-0002, VR-0011 and
    incidents I-0003 to I-0007. Its rules are campaign rules R16, R19 and R20.
- **Rulings of 2026-11-18,** in the campaign plan's twelfth set. The verification and
  sandbox saves are tools, used as is sensible, with experimental procedures allowed.
  The career is very conservative. All sensible learning is adopted. Standing push
  and merge authority covers Codex lines. Exact document wording is the line's to settle.

## For the receiving Codex line

1. Follow the startup protocol in `CLAUDE.md`.
2. Run cargo and the gate as `RUSTC_WRAPPER= ...`. Stage files before the gate, and
   commit only on its exit status.
3. Before any game run, read `tools/sepratron/README.md` and rule R20.
   - Every launch first clears crewed vessels near the pad or runway, through
     `tools/phase0/launch_site.py`.
   - kRPC's `load` hangs, so the operator loads saves from the game's menu.
   - A Flight Results dialog after a lost active vessel blocks the next launch until
     the operator closes it.
   - `set_velocity` takes its frame literally. Place falling vessels with `set_flight`,
     and read back every commanded state.
   - Announce each run's start, ask for no input during it, and keep the measured
     vessel active so the camera follows it.
4. The debug service is allowed only in "Ikria Physical Verification". Never load or
   change the career "Ikria Automated Space Agency" without the operator's word.

## Open work, in recommended order

1. **Diagnose I-0009 before further TP-0003 flights.** The bridge exited on a cycle
   deadline after 3,558 acknowledged normal-hop commands. Current diagnostics do not
   identify the stalled operation. Add cycle and operation context while preserving
   deadlines and discarding cancelled connections. Do not extend the 700 ms bound to
   force a pass. The failed journal is retained and replay refuses it. Normal completion
   and refused-boost flight remain owed. The two pad checks passed. I-0008 remains open
   for placement reliability. Manual editor placement admitted the empty PreLaunch
   article once, preserved in `ipv-flight-fix-manual-pad-20261122`. The operator loads
   it through the menu when testing resumes. Inspect the actual state again. Do not
   weaken PreLaunch or repeat an uncertain command. `place --uncrewed` is still not
   verified live. Tested fault fixtures use `run_flight.py --program-body`.
2. **T12, the sandbox qualification.** A campaign-valid control source, attitude
   actuation, broader abort tests, repeatability, crewed mass and mission science.
   E1 and E2 designs remain open.
3. **Dependabot pull request 47,** `sha2` 0.10.9 to 0.11.1, opened 2026-11-20. Review it
   like any change before merging.
4. **R20 in the Rust harness.** The Python tools enforce it. The harness design's
   enforcement table says the Rust side is to adopt it.
5. **Sepratron follow-ups.** Fly the ejectable cockpit, a hybrid ascent and a
   full-thrust soft landing, and settle the surface-feature multiplier and the 25
   unverified location names when a save holds such subjects.
6. **T14** stays paused after step 4 of TP-0001, until a vessel meets a Green Monolith.
7. **Later work.** Multi-vessel scheduling, phase modules, in-flight program
   replacement, calibrated timing admission, the mission controller and bridge
   batching remain unimplemented.

## Last observed game state

- **The career** had only the start node, zero science and 10,000 funds on 2026-11-17,
  and was not loaded since. Its file hash still matched VR-0010 on 2026-11-22.
- **The verification save** has its whole tree researched and purchased since the
  Sepratron program. Jebediah was restored after I-0007, and all four kerbals were
  Available on 2026-11-18 and again on 2026-11-22. Checkpoints include "ipv-before-sepratron-program",
  "ipv-after-sepratron-program" and "ipv-jeb-restored". Debris from tests remains in
  orbit and on Minmus and the Mun.
- On 2026-11-22 the game returned to the verification Space Center after the empty
  article's settled landing and recovery. All four kerbals were Available, no crew
  occupied either launch site, and the career hash was unchanged. Five debug flags
  were false and no debug setter was used. A recovery dialog may be visible, which
  telemetry does not establish. Retain `ipv-before-flight-fix-verification-20261122`
  and the manual-pad checkpoint above. Editor changes to the craft affected only its
  root version and description, with no part-configuration change.

## Worktrees

Preserve `tmp/worktrees/feat-codex-flight-control`, which holds Codex's raw logs and
rehearsal tooling. Preserve `test-codex-flight-fix-verification` and
`test-codex-flight-fix-live` for the latest raw logs and probes. Earlier merged
worktrees may be removed after checking their retained evidence. The bridge's sibling `ikria` symlink supplies its records, and its
workflow pins `504d4c4`.

## Other open items

- Main is protected and needs a pull request with a green `CI result`.
- Scripts match template `8a79622`, except project checks.
- Cite and consider Massachusetts Institute of Technology Lincoln Laboratory's KSPDG.
- Measure game determinism rather than infer it from software replay.
- Architecture notes retain derived-constant tests, the post-ignition PreLaunch abort
  edge case, shared loader faults, bridge latch defence and unjournalled invalid bytes.

## Environment

KSP 1.12.5 build 03190 with both expansions, kRPC 0.7.0 and Module Manager 4.2.3, under
Rosetta 2, whose removal in macOS 28 is a risk. kRPC uses ports 50000 and 50001, and the
bridge uses 50120. The kRPC Python client 0.7.0 is in `tmp/krpc-venv`, local scratch.
The bridge is at `~/projects/keleusma/ikria-bridge`, GPL-3.0-only, with no outside contributions. Rookie and Sandbox saves are read only.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

A valid checkout is clean, its newest commit describes this state, and its gate exits
zero. The gate passes on macOS with 158 Rust tests, 51 Phase 0 Python tests and
the gate's own 12, none ignored. Bridge validation passed with three Rust tests.
Main checks passed remotely at `7907697` and `ef85dd8`. The current pull
request records its required checks. Report checks not run and later commits.

## Mailbox

None assigned. The blog line is reached through its running Claude Code session and
the admin line through `Op55hi-admin-polybasite`, so a Codex line coordinates through
the operator. Article requests remain context.
