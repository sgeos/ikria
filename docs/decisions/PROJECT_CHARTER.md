# Project Charter

> **Navigation**: [Decisions](./README.md) | [Documentation Root](../README.md)

**Status.** Decided.
**Date.** 2026-11-13.
**Decided by.** The operator, in answer to the open questions of the same date, recorded
by the admin session.

## Context

Ikria is a Keleusma technology demonstrator. It demonstrates stream-processing control in
a textbook sandbox simulator, Kerbal Space Program 1, where mistakes can be undone and
crashes have no physical-world impact. The operator describes it as a reverse form of the
research in the blog article A386 on multi-agent space simulation, which asks what a
simulation that does not cheat its physics would need. Ikria instead takes an existing
simulator, states its limits, and uses it to exercise control software.

## Decision

1. **Hot loading** follows the Keleusma specification, which the operator regards as
   solving it nominally. Problems found in practice are reported upstream, where they may
   drive language fixes and developments. Ikria does not invent its own swap semantics.
2. **No manual flying.** Every flight input comes from Keleusma control loops. Quickloads
   and reverts are allowed. Ikria is a control-loop demonstrator, not a no-reload
   challenge.
3. **Keleusma first.** As much of the project as reasonably possible is written in
   Keleusma, including program-level logic, and the share is expected to grow with each
   public release.
4. **A simulated space agency.** Ikria plays its own save files. Craft are designed and
   tested in a sandbox save, and worked designs are then used to complete objectives in a
   career save, subject to that mode's constraints. Craft designed in the sandbox must be
   buildable in the career save. Agentic AI drives the broader programme and builds the
   craft, and Keleusma control loops fly them.
5. **Stepping.** One control step per physics tick if reasonably possible, and otherwise
   as fast as is reasonable.
6. **Licensing.** The bulk of the code is 0BSD. If the bridge must be GPL because of
   kRPC, it lives in a small separate repository.
7. **Kerbals on EVA are full agents.** All craft have limits. Crew must be kept alive.
   Uncrewed craft must meet their mission requirements, which may include permanent
   insertion into orbit or permanent placement on a distant body.
8. **Public from the start.** The repository may start public. Outreach, such as
   summaries on social media and videos, is not mainline work but is expected to become a
   project concern.
9. **Milestones** follow career mode's established flow and cadence, and are still to be
   worked out.

## Consequences

- The record of each mission states which control loops flew it, and should show each
  quickload and revert, so that readers can see what the loops achieved. The second part
  is the admin session's recommendation, not an operator rule.
- One step per physics tick through kRPC is uncertain, because kRPC is a network
  protocol with its own latency. If it cannot keep up, the options are a slower fixed
  rate or an in-game component, which would belong in the GPL bridge repository.
- Agentic AI building craft needs a path to create craft. kRPC 0.7.0 exposes an editor
  vessel, and craft files are text, but neither path has been checked.
- Going public needs the Appendix B check first, which the seeding session could not do,
  and template checklist step 16, the history and credential scan.
- Publishing game footage and craft files rests on the game's terms and the publisher's
  content policy. The operator notes that many people publish KSP videos. The policy has
  not been read.
