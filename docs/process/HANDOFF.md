# Handoff

> **Navigation**: [Process](README.md) | [Documentation Root](../README.md)

Keep this state file within 8 KB. History lives in git. Account for removed open
items in the commit message.

## State on 2026-11-22

Written by Codex on `test/codex-flight-fix-verification`, based on refreshed main
`4ebd894`. Bridge main is `ef85dd8`. The operator loaded the verification save and
authorized continued work and reports. Publication follows the standing authority.
The branch's pull request and merge ancestry record its publication state.

- **Milestone.** Every task of [`V0_1_0.md`](../roadmap/V0_1_0.md) is done except T12,
  partial, and T14, paused. Since the last Codex handoff, these merged.
  - **T8 and T9.** `crates/ikria-save` reads saves, and `crates/ikria-ledger` gives the
    science ledger, 3,104 subjects and 217,827 science on Hard. VR-0005 was corrected
    from 217,818.
  - **Flight-control defects fixed.** All seven, reviewed twice, in pull request 44. A
    refused output selects the distinct `programs/abort.kl`. See
    [FLIGHT_CONTROL.md](../architecture/FLIGHT_CONTROL.md). Not yet flown.
  - **VR-0012.** The corrected harness completed a 50-sample monitor with exact replay.
    Armed checks are blocked by preparation, I-0008. TP-0003 and tested fault fixtures
    are ready. No flight or ignition occurred on 2026-11-22.
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

1. **Resolve I-0008 and resume TP-0003.** Crewed placement followed by crew removal
   left the empty article Landed. Direct empty-crew placement displayed No Control
   and timed out. A second request also timed out. Snapshots after each showed no
   test article. The operator was asked to cancel any remaining warning. Check the
   actual state before any further command. Do not weaken the PreLaunch requirement.
   The guarded `place --uncrewed` option permits a 180-second warning-response window
   but has not completed live placement. Normal, pad-abort, refused-pad and refused-boost
   checks are still owed. `run_flight.py --program-body` selects the test sources.
   Their exact files are covered by an offline same-sample abort regression test.
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
- On 2026-11-22 the game was at the verification Space Center after the failed
  placements, with kRPC responsive, no test article on the pad and no crew at either
  launch site. Five debug flags were false before and after, and no debug setter was
  used. The new checkpoint is `ipv-before-flight-fix-verification-20261122`. A warning
  dialog may remain, which telemetry cannot establish.

## Worktrees

Preserve `tmp/worktrees/feat-codex-flight-control`, which holds Codex's raw logs and
rehearsal tooling. Codex's other three worktrees under `tmp/worktrees` are merged and
may be removed. The bridge's sibling `ikria` symlink supplies its records, and its
workflow pins `504d4c4`.

## Other open items

- Main is protected and needs a pull request with a green `CI result`.
- The scripts match template commit `8a79622`, except Ikria's own project checks.
- Cite and consider Massachusetts Institute of Technology Lincoln Laboratory's KSPDG.
- Measure game determinism rather than infer it from software replay.

## Environment

KSP 1.12.5 build 03190 with both expansions, kRPC 0.7.0 and Module Manager 4.2.3, under
Rosetta 2, whose removal in macOS 28 is a risk. kRPC uses ports 50000 and 50001, and the
bridge uses 50120. The kRPC Python client 0.7.0 is in `tmp/krpc-venv`, local scratch.
The bridge is at `~/projects/keleusma/ikria-bridge`, GPL-3.0-only, with no outside
contributions. Rookie and Sandbox saves are the operator's and are read only.

## Validity

```sh
git status --short
git log --oneline -5
RUSTC_WRAPPER= ./scripts/check.sh
```

A valid checkout is clean, its newest commit describes this state, and its gate exits
zero. The gate passes on macOS with 158 Rust tests, 51 Phase 0 Python tests and
the gate's own 12, none ignored. Bridge validation passed with three Rust tests.
Startup main checks passed remotely at `4ebd894` and `ef85dd8`. The current pull
request records its required checks. Report checks not run and later commits.

## Mailbox

None assigned. The blog line is reached through its running Claude Code session and
the admin line through `Op55hi-admin-polybasite`, so a Codex line coordinates through
the operator. Article requests remain context.
