# Open Questions on the Brief

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

Raised by the admin session on 2026-11-13 against the operator brief in
`docs/process/HANDOFF.md`. Each needs the operator's answer before the design document is
final. Record each answer here with its date, or move it to `docs/decisions/` as a
decision record, and delete the question once it is settled.

## Mission controller

1. **Who drives it.** Is the MCP side for agents such as Claude acting as flight director,
   for the operator, or both? Which actions may an agent take without the operator's
   approval, such as launching, staging or spending career funds?
2. **What its driver loops are written in.** Are the space-program-level loops Keleusma
   stream processors too, which would make the demonstrator stronger, or host code?

## Harness

3. **Process layout.** One host process running many Keleusma programs, or one process
   per craft? Who owns the shared clock and the step rate?
4. **The stepping contract.** One step per physics tick, or a fixed control rate below
   it? kRPC adds latency between the game and the host.
5. **Hot loading.** When may a swap happen, such as only between steps? What happens to
   the old program's state: discarded, handed over through a declared record, or
   migrated? Who may trigger a swap: the controller, an agent through MCP, or a file
   watch? Must the Keleusma verifier accept the new program first, and what flies the
   craft if it rejects it?
6. **Personnel.** Are kerbals on EVA first-class agents? The jetpack works only for the
   active vessel, which limits concurrent EVA.

## Bridge

7. **kRPC and its client.** Is kRPC the settled bridge? Is there a maintained Rust kRPC
   client, or must one be generated from kRPC's service definitions? kRPC is GPL-3.0, so
   check the client libraries' licences before linking them into a 0BSD project.

## Sandbox and career

8. **Promotion.** How does a craft move from the sandbox save to the career save, given
   that career mode limits parts by research and funds?
9. **Honesty of career runs.** Are quickloads and reverts allowed in career missions? A
   public demonstrator should state the rule, and the record should show every use.
10. **Manual input.** May a human fly any part of a career mission, and is it recorded?

## Public presentation

11. **When to go public.** The template keeps the repository private until checklist step
    16, the history and credential scan. Is the plan public from the first milestone, or
    later?
12. **Game content.** Screenshots, video and craft files shown publicly fall under the
    game's terms and its publisher's content policy, which have not been checked.
13. **Success criteria.** Which missions make the first milestone, for example orbit,
    a Mün landing and return, or a crewed EVA?
