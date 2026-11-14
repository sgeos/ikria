# Agency Record

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Decided in scope, 2026-11-14. The operator decided the document set,
including the additions this document proposed. The designations and the directory
layout are proposed. The format of each document follows a published blog article on
the matching industry practice, as R11 already does for incident reports. Until an
article exists, the section lists below are provisional.

Ikria is to be run as a serious model of a space agency. This document defines what
the agency records, why, from what data, and where. It extends rules R7, R8 and R11 of
[`CAMPAIGN_PLAN.md`](CAMPAIGN_PLAN.md).

## Operator brief, 2026-11-14

**Decided.** The operator asked for the following.

1. **A specification sheet for each vehicle.** Modelled on engineering practice, with a
   bill of materials, the Keleusma control loops that fly it, and a history section on
   when a comparable vehicle could first have been built in the real world. The format
   is to rest on its own blog post.
2. **Tracked missions, based on industry practice.** Each mission notes how the game
   differs from the real world in ways that would affect a similar real mission. It also
   lists what follows from using the mission as a model for a real one, for example Eve
   as an analogue of Venus and the physical differences that matter there. The format is
   to rest on its own blog post.
3. **The space program itself tracked.**
4. **Financials tracked.**
5. **Anything else a serious space agency would keep.** The operator expects the list
   above to be incomplete. The operator accepted this document's additions the same
   day, which are the fleet and object registry, crew records, the science ledger, the
   configuration and software catalogue, and public summaries later. The program and
   financial records also rest on their own blog article, requested that day.
6. **A methodology that holds up to scrutiny.** The initiative is to be serious enough
   that professionals who engage with it seriously can take its methodology seriously.

## Principles

- **Open to professional scrutiny.** Each document states its method, its sources, its
  assumptions and its limits, so that a professional reader can check it and disagree
  with it on the evidence.
- **Grounded in practice.** Each document type follows a published article from the
  blog line on how agencies and industry keep it. The article is cited in the template.
- **Generated, not typed.** Numbers come from the save file, kRPC telemetry, the
  harness's logs and the game's data files, through scripts kept in this repository.
  Prose interprets the numbers but never restates them by hand.
- **Two clocks.** Every entry carries the game's universal time and the real date in
  Coordinated Universal Time.
- **Marked knowledge.** Facts, inferences and hypotheses are marked, as everywhere in
  this repository. Comparisons with the real world name their sources.
- **Bounded comparisons.** A comparison with the real world says how a comparable real
  mission or vehicle would differ, and what the game cannot show. It describes the
  game's fidelity, and claims nothing beyond it.

## Document set

| Record | One per | Written | Main sources | Format basis | Standing |
|---|---|---|---|---|---|
| Vehicle specification sheet | Vehicle design and version | Before qualification, revised with each version | Craft file, part data, sandbox qualification | Blog article requested 2026-11-14 | Decided, provisional sections below |
| Mission plan | Mission | Before the flight readiness review | Plan, trajectory tools, sheets of the vehicles flown | Blog article requested 2026-11-14 | Decided, provisional sections below |
| Mission report | Mission | After recovery or end of mission | Telemetry, save file, harness logs | Same article as the plan | Decided, provisional sections below |
| Incident report | Event under R11 | Within the mission's close-out | Telemetry, save file | A391 on the blog, drafted | Per R11 |
| Campaign ledger | Campaign | After every mission | Save file | Campaign plan criteria | Defined by C1 to C18 |
| Program status report | Kerbin year of game time | At each year's end | All of the above | Blog article on program and financial management, requested 2026-11-14 | Decided |
| Financial ledger and statements | Transaction, and per mission and per year | Continuously, and summarized per mission and per year | Funds read through kRPC and the save file | Same program and financial article | Decided |
| Launch manifest and master schedule | Campaign | Revised at each planning cycle | Plan, sequence, launch windows | Same program and financial article | Decided |
| Fleet and object registry | Vessel or object | At launch, at each change of state, and before any deletion under R13 | Save file, kRPC | Practice of national registries of launched objects *(recall)* | Decided |
| Crew records | Kerbal | At hire or rescue, after every flight | Save file roster | To be researched | Decided |
| Science ledger | Subject | After every recovery or transmission | Save file science subjects | C4 | Decided |
| Configuration and software catalogue | Craft file and Keleusma program version | At each change | Repository, hashes under R7 | Configuration management practice, and the vehicle article | Decided |
| Risk register and lessons learned | Risk or lesson | At each review | Plans, reports, incidents | Same program and financial article | Decided |
| Reviews | Design review, flight readiness review, post-flight review | At each gate | The documents above | Mission article | Decided |
| Public summaries | Mission | After the mission report | Mission report | Charter item 8 | Decided, later |

## Vehicle specification sheet, provisional sections

1. **Identification.** Designation, version, status, the missions it serves, and the
   craft file hash.
2. **Purpose and requirements.** What it must do, traced to the mission plans and the
   completion criteria.
3. **Configuration.** A drawing or render, stages, and the assembly sequence.
4. **Bill of materials.** One row per part type, with the quantity, unit cost, mass,
   technology node and purchase status, generated from the craft file and the game's
   part data. The rows sum to the vehicle's cost and dry and wet mass.
5. **Performance.** Δv and thrust-to-weight ratio by stage, mass budget with margin,
   heat tolerance, and the facility limits it must fit, such as mass, size and part
   count.
6. **Control.** The Keleusma programs that fly it, with their versions and hashes, the
   flight phases each covers, the telemetry each reads and the commands each sends, the
   stepping rate, the aborts each can command, and how failures are handled.
7. **Qualification.** Sandbox flights under R4, with their results, and the pad check.
8. **Flight history.** Every career flight of the design, with its outcome.
9. **Disposal.** What becomes of each stage under rule R13 of the campaign plan, which
   may be recovery, deorbit, a second purpose such as a relay, or a logged deletion.
10. **History.** When a comparable vehicle could first have been built in the real world,
   with its closest historical counterparts. Also the differences between the game's
   parts and real hardware that the comparison depends on, such as the scale of the
   planets, the density of parts and the absence of component failures.

## Mission plan and report, provisional sections

**Plan.**
1. **Identification.** Designation, objectives, success criteria, and the completion
   criteria and contracts served.
2. **Concept of operations.** Phases, vehicles, crew and timeline.
3. **Trajectory and windows.** Launch window, transfers, and the method used to compute
   them.
4. **Budgets.** Δv, electric charge, monopropellant and propellant for extravehicular
   activity under R6, and funds.
5. **Flight rules and aborts.** The rules for each phase, the abort modes and the
   go and no-go criteria.
6. **Risks.** The risks of this mission, drawn from and added to the register.
7. **The game and the real world.** How a comparable real mission would differ, and
   what follows from using this mission as a model of one. The reference table below
   gives the starting values.

**Report.**
1. **Results** against each objective and success criterion.
2. **Timeline** from telemetry.
3. **Anomalies and incidents,** with references to incident reports.
4. **Costs** against plan, from the financial ledger.
5. **Science and criteria** gained, from the ledgers.
6. **Lessons learned,** entered in the register.
7. **The game and the real world, revisited.** What the flight showed about the
   comparison.

## The game and the real world, reference values

*Starting values, mostly recalled, to be checked against sources in each mission plan
before use.* The game's solar system is built at about one tenth of the real one's
scale, with dense planets that keep surface gravity near Earth's.

| Body in the game | Real analogue | Differences that matter for a comparable mission *(recall)* |
|---|---|---|
| Kerbin | Earth | Radius 600 km against 6,371 km. Low orbit near 2.3 km/s against 7.8 km/s, so the Δv to reach orbit is about a third of Earth's. |
| Mun | Moon | Much smaller and closer to its planet, so transfers are shorter and cheaper. |
| Minmus | None close | A small icy moon with flats. It has no real counterpart near Earth. |
| Eve | Venus | Surface gravity about 1.7 g against 0.9 g, and pressure about 5 atmospheres against about 92. Venus's surface near 737 K is far hotter, and the game models nothing like its corrosive clouds. |
| Duna | Mars | A much smaller planet with a thinner but proportionally denser atmosphere, so parachutes behave differently. |
| Moho | Mercury | Close to the Sun and expensive in Δv, as Mercury is. |
| Jool and its moons | Jupiter and its moons | No radiation belts, which dominate real missions to Jupiter. |
| Laythe | An ocean moon such as Europa, with an atmosphere like Titan's | A breathable atmosphere and surface oceans, which no real moon has. |
| Dres and Eeloo | Ceres and Pluto | Small, cold and distant, as their analogues are. |

**Physics the game does not model.** *Recall.* The game uses patched conics with one
body's gravity at a time, so it has no Lagrange points, perturbations or three-body
trajectories. Vessels outside the physics range move on rails. It has no radiation
environment, no life support beyond electric charge, and no random failure of parts. The
signal network has no light-time delay. Heating, aerodynamics and engine behaviour are
simplified. Each mission plan names which of these affect its comparison.

## Designations, proposed

| Kind | Form | Example |
|---|---|---|
| Vehicle design | V-, three digits, then a version letter | V-001A |
| Vehicle built for a flight | Design, then a serial | V-001A-03 |
| Mission | M-, four digits | M-0001 |
| Incident | I-, four digits | I-0001 |
| Review | The mission, then the review type | M-0001-FRR |

## Where the record lives, proposed

A top-level `record/` directory, created in Phase 0, holds the generated documents. Its
`data/` directory holds the machine-readable sources from which they are built.
`templates/` holds the templates once the blog articles exist. One directory per kind
holds the documents: `vehicles/`, `missions/`, `incidents/`, `program/`, `finance/`,
`fleet/`, `crew/`, `science/` and `software/`.

## Open technical questions for Phase 0

- **Transaction reasons.** The game records why funds change, such as a contract, a
  recovery or a purchase *(recall)*. Whether kRPC exposes the reasons, or the ledger
  must infer them from snapshots before and after each event, decides how the financial
  ledger is built.
- **Telemetry volume.** One step per physics tick for several vessels produces a large
  log. The record keeps summaries in the repository and the full logs elsewhere, and
  where is to be decided.
