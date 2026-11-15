# Campaign Sequence

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Proposed, 2026-11-14. This is a prospective plan, and it is expected to
change at each checkpoint below. It turns the phases and the technology order of
[`CAMPAIGN_PLAN.md`](CAMPAIGN_PLAN.md) into a concrete order of research, purchases
and facility upgrades. It also says where the science and the funds are to come from.
It covers the early game in detail, because the early game is where the campaign is
tightest. Sources are marked as in the campaign plan. *(files)* is the installed game's
data, *(web)* the official wiki read on 2026-11-14, and *(recall)* memory not yet
checked.

## Operator rulings, 2026-11-14

- **Return on investment.** The program is run on return on investment.
- **Science before cash.** Science is spent to unlock every node of the tree before any
  surplus is converted to funds. Surplus science is therefore banked until the whole
  tree is researched, and only then may a strategy such as Patents Licensing turn
  science into funds.
- **The booster first.** A first-stage booster that takes off vertically and lands on
  the runway is one of the top priorities, as
  [`VTHL_PROGRAM.md`](VTHL_PROGRAM.md) sets out.

## The early-game problem

The operator expected the early game to be the hardest part. The facility gates bear
that out.

- **Research is capped.** At its first level, the Research and Development facility
  researches only nodes costing 100 science or less *(web)*. That window holds 20
  nodes beyond the start, costing 1,188 science together, and 177 parts whose entry
  costs total 514,730 funds, against 4,655,650 for the whole tree *(files)*. Leaving
  the window costs 676,500 funds for the second level *(web)*.
- **Kerbals stay aboard away from Kerbin.** At its first level, the Astronaut Complex
  allows extravehicular activity only on Kerbin *(web)*. Reports outside the vessel,
  flags and rescues away from Kerbin need the second level, at 112,500 funds. Surface
  samples need that and the second level of Research and Development *(web)*.
- **Contracts are few.** Mission Control at its first level allows 2 active contracts,
  and 7 at its second, which costs 75,000 *(web)*.
- **Vehicles are small.** The launch pad and the runway cap a vessel at 18 tonnes, and
  the assembly buildings at 30 parts, until their second levels *(web)*.

*Inference.* Science fills the research window by about the first landings on the Mun
and Minmus, as the stage table below shows. Leaving the window takes about 940,000 funds
of upgrades alone. The early game is therefore a funds problem, and the plan is built
around closing it.

## Facility costs

From the official wiki's facility pages *(web)*. The wiki shows no difficulty
multiplier on these prices.

| Facility | Level 2 | Level 3 | What the next level opens |
|---|---|---|---|
| Research and Development | 676,500 | 2,535,000 | Nodes to 500, then all, and surface samples with the Astronaut Complex |
| Astronaut Complex | 112,500 | 485,000 | Extravehicular activity and flags away from Kerbin, then unlimited crew |
| Mission Control | 75,000 | 282,000 | 7 active contracts, then unlimited, and manoeuvre nodes with the Tracking Station |
| Launch pad | 75,000 | 280,000 | 140 tonnes and 36 m, then no limit |
| Runway | 75,000 | 282,000 | 140 tonnes, then no limit |
| Vehicle Assembly Building | 337,500 | 1,267,500 | 255 parts and basic action groups, then no limit |
| Spaceplane Hangar | 337,500 | 1,267,500 | 255 parts, then no limit |
| Tracking Station | 150,000 | 563,000 | Ground network 50 G and orbit patches, then 250 G |
| Administration Building | 150,000 | 563,000 | 3 strategies, then 5 |

Every upgrade together costs 9,514,000 funds, and every part 4,655,650, so C2 and C10
together need about 14.2 million funds before any launch is paid for.

## Stages of the early game

The science column is the first collection of each subject at its base value, from the
game's experiment values and the wiki's multipliers. Hard is 60 percent of Normal,
because the reward multiplier scales each subject's cap. That was measured on
2026-11-14 in the sandbox, which is on the Hard preset. An extravehicular report from
the launch pad had a cap of 1.44, which is its base of 8 times Kerbin's landed
multiplier of 0.3 times 0.6.
Cumulative research assumes every listed node is taken.

| Stage | Research, cumulative cost | Buy | Upgrade | Science, Hard | Exit |
|---|---|---|---|---|---|
| E0, day one | Then Engineering 101, Basic Rocketry, Survivability, 25 | Nothing, since the start parts are free | None | About 32 | First launch flown, payouts measured |
| E1, sounding rockets | Stability, General Rocketry, 63 | Hammer, Swivel, small tank, decoupler, thermometer, barometer, about 12,000 | None | About 102 | Space reached and recovered |
| E2, aircraft and booster | Aviation, Basic Science, 153 | Juno, Mk1 cockpit, fuselage, wings, elevons, gear, intake, Science Jr., first probe core, about 26,000 | None | About 128 | Kerbin flown low and landed in six biomes, and the booster's development steps V1 to V3 done in the sandbox |
| E3, the Mun and Minmus from space | Advanced Rocketry, Flight Control, General Construction, Electrics, Fuel Systems, 468 | Terrier, Reliant, tanks, high gain antenna, battery, OKTO, solar panels, reaction wheel, about 23,000 | Mission Control to 2 | About 420 | Crewed passes of both, returned |
| E4, first landings | Landing, Advanced Flight Control, Miniaturization, Space Exploration, 828 | Landing struts, lander can, EVA Experiments Kit, first rover wheel, about 14,000 | Astronaut Complex to 2, launch pad to 2 if needed | About 324 for one biome on each | Crewed landing and return on both |
| E5, biome campaign | The rest of the window as missions need it, 1,188 | Parts as each mission needs them | None | Up to about 4,070 for every landed biome of both, banked | Funds for Research and Development level 2 |
| E6, leaving the window | Research and Development to 2, then the 160 tier | See the later stages | Research and Development to 2 | | The early game ends |

**Notes on the stages.**
- **E0.** Reports on foot at about 12 of the Space Center's location biomes, a crew
  report and Mystery Goo on the pad and on the runway, and one hop on the Flea booster.
  Rolling out and recovering from the pad refunds everything, so the day costs almost
  nothing.
- **E1.** Reports, Goo, thermometer and barometer readings flying high, in space low
  and in space high over Kerbin, one splashdown, and the readings on the pad and the
  runway. The record contracts for altitude, speed and distance belong here.
- **E2, the booster.** The aircraft program's runway landings are the booster's first
  development step. Steps V1 to V3 of [`VTHL_PROGRAM.md`](VTHL_PROGRAM.md) fly in the
  sandbox during E2, so that the booster is qualified before the first launch to orbit.
- **E2.** A crewed aircraft with a scientist aboard. Under R2 any kerbal may sit in the
  cockpit, since the Keleusma loop flies. On Kerbin the scientist may leave the
  aircraft after each landing, take reports, and reset Mystery Goo and the Science Jr.
  for the next biome. The level a scientist needs to reset experiments is *(recall)*.
  Flying low over a biome pays little, so the aircraft lands in each biome it can.
- **E3.** Probes reach the Mun from the first ground network with the smallest antenna,
  and Minmus needs the high gain antenna *(inference from recalled antenna powers)*.
  Mission Control comes first among upgrades, because 7 contracts at once instead of 2
  is the largest early gain in income.
- **E4.** The flyby of the Mun comes first, and the first landing is on Minmus,
  following the plan's community practice. The Astronaut Complex goes to level 2
  before the first crewed landing, for flags, reports outside the vessel and the world
  firsts that depend on them. The 18 tonne limit may force the launch pad upgrade here.
- **E5.** Landers that hop between biomes, or several landers, collect the 26 landed
  biomes of the Mun and Minmus. That yields about 4,070 science on Hard, far beyond the
  window's 1,188. Under the science ruling the surplus is banked, not converted, and
  pays for the 160 and 300 tiers as soon as Research and Development reaches level 2.

## Funds plan to leave the window

All figures are projections, to be replaced by measurement at the checkpoints.

**Costs before Research and Development level 2.**

| Item | Funds |
|---|---|
| Mission Control to 2 | 75,000 |
| Astronaut Complex to 2 | 112,500 |
| Launch pad to 2, if needed | 75,000 |
| Research and Development to 2 | 676,500 |
| Part purchases for E1 to E4 | about 80,000 to 100,000 |
| Launches net of recovery, about 40 to 60 flights | about 150,000 to 250,000 *(estimate)*, lower once the booster flies |
| **Total** | **about 1.2 to 1.3 million** |

**Income.**
- **Start.** 10,000 funds.
- **Surplus science, not before the tree is complete.** The Patents Licensing strategy
  converts a share of science from field work into funds, at about 84 to 104 funds for
  each point of science by the wiki's figures *(web, possibly at Normal rates)*. Under
  the science ruling it is not used before every node is researched, so it contributes
  nothing here. The first draft of this plan counted about 280,000 to 340,000 funds from
  it, which the ruling removes.
- **Contracts.** The existing Normal career recorded a survey of Kerbin paying 25,937
  in advance and on completion, and a station paying 119,150 *(files)*. On Hard these
  become about 15,600 and 71,500. A rescue has a base of 92,000 before the body and
  difficulty multipliers *(files)*, so about 55,000 on Hard near Kerbin. A rescue also
  brings a kerbal without the rising cost of hiring.
- **World firsts.** Each progress node has a base of 80,000 funds before its weight and
  multipliers, and 20 percent of it is paid passively without a contract *(files)*. The
  weights are not in the files, so the payouts are measured from E0 onward.

**The gap.** Contracts, world firsts and recovery must supply about 1.2 million funds
before E6. At the anchors above, that is about 75 survey-sized contracts, or about 22
rescues, or a mix. The booster lowers the launch costs in the table, which narrows the
gap. *Inference.* This is achievable within E1 to E5 if Mission Control is upgraded
early, the booster flies early, and rescues are available. It is the main uncertainty
of the early game.

## Design principles for landers and stages

The operator set these out on 2026-11-14.

- **Shed the bulk before the ascent.** On a body with low gravity, a lander can leave its
  heavy descent stage on the surface and lift only a small science and command module,
  either to orbit for docking or straight onto a transfer home. Small solid motors can
  give enough Δv for that there. *(files)* Kerbal Space Program 1 has them. The FM1
  Mite is in Propulsion Systems, at 90 science and inside the first research window, and
  the Sepratron I and the F3S0 Shrimp are in Precision Propulsion, at 160.
- **Pair the solids with control.** The more realistic design pairs small solid motors
  with reaction control and small engines that can be throttled. *(files)* The Ant and
  the Spark are in Propulsion Systems and the RV-105 thruster block in Advanced Flight
  Control, both at 90 science. The Spider, the Twitch and the Puff are in Precision
  Propulsion, at 160.
- **Leave what earns its place.** A lander left on a surface is acceptable when an
  analysis of return on investment supports it. Fitted with solar panels, batteries and
  relay hardware, it becomes a surface relay or a historical base under rule R13 of the
  campaign plan.
- **One craft, several objectives.** Craft that meet several objectives at once are the
  goal, as the operator set out on 2026-11-14. A lander left behind may also meet an
  outpost construction contract or the world first for a base. The operator expects it
  would need to be fitted to each contract's requirements, which the mission plan checks
  against the contract.
- **Spent stages can be infrastructure.** A stage left in orbit with relay hardware, and
  serving as a relay, is not debris.

## Green Monoliths

The operator asked on 2026-11-14 for an investigation of the report that monoliths can
unlock technology nodes without the facility upgrades those nodes otherwise need.

**What is established.**
- *(web, the official wiki's Green Monolith page)* Since version 1.2, each save places one
  Green Monolith at a random spot on every body except Jool and the Sun, 15 in all.
  Bringing a vessel or a kerbal close enough to one counts as its discovery. In career
  and science mode the discovery is announced among the world firsts and unlocks a
  research node, and the wiki's example names Experimental Motors, a 1,000-science
  node.
- *(web)* A Green Monolith is located through KerbNet, where it shows as an anomaly.
- *(files)* KerbNet's anomaly detection comes with these parts.
  - **First window.** The OKTO probe core, at 90 science, has a detection value of 0.06.
    The Stayputnik has KerbNet but no anomaly detection.
  - **160 science.** The HECS core has 0.12, and the avionics nose cone 0.24.
  - **300 science.** The RoveMate rover core has 1.0.

**What is not established.** How the unlocked node is chosen, and whether the unlock
ignores the Research and Development cap, the node's cost and its prerequisites. The
wiki's example suggests that the cap does not apply, but one example proves nothing.
Neither question can be settled from the game's configuration files. Reading its code
would be reverse engineering, which the charter excludes, so they are settled by test.

**Why it matters.** If a monolith unlocks a node above the first window's cap of 100
science, Kerbin's monolith alone could open a node of the 160 tier or above long before
Research and Development level 2, which costs 676,500 funds. Kerbin's, the Mun's and
Minmus's monoliths are all within reach of the early game. Even if the cap applies, the
15 monoliths are 15 free nodes, and each is an anomaly for C13.

**Test plan.** *Proposed.* The question needs career mechanics, which the sandbox lacks.
It must not be learned in the campaign's own career, because a monolith's random place
and its node belong to that save. A third save, a test career on Hard used only for
mechanics, would carry it.
1. Research the OKTO, then put an OKTO probe in a low polar orbit of Kerbin and scan for
   anomalies through KerbNet until the Green Monolith appears.
2. Reach the monolith by aircraft or by rover, and record which node unlocks, the
   Research and Development level at the time, and the node's cost and prerequisites.

In the campaign itself, R9 allows the monolith's position to be known only once a
KerbNet scan reveals it, so the search is part of the plan.

**Ruling, 2026-11-14.** The operator ruled that monoliths are used to unlock the final
nodes of the tree, and the nodes that hold no parts, Nanolathing and Experimental
Motors. The operator understands the granted node to be random, which the test above is
to confirm.

**How the ruling is carried out.** *Proposed.*
- **A reserved set with slack.** The tree is researched in full except for a reserved set
  of 13 nodes, which are Nanolathing, Experimental Motors, and the last and most
  expensive nodes the campaign needs least. There are 15 monoliths, so two are slack for
  accidents, as the operator ruled on 2026-11-14.
- **Discovery waits.** No monolith is discovered until every node outside the reserved
  set is researched. If the grant is random among the locked nodes, every grant then
  lands in the reserved set.
- **Keep-out zones.** A discovery happens by proximity, so once a KerbNet scan reveals a
  monolith, missions keep away from it until the endgame. The keep-out distance is to be
  set from the test. Each mission plan near a revealed monolith states its closest
  approach.
- **Accidental discovery.** A monolith discovered before the endgame is an incident
  under R11. The report records the node granted, and adjusts the reserved set. If the
  granted node was in the set, the set shrinks by one. If it was not, the set shrinks by
  one more node, the cheapest in it, since one fewer monolith remains for the endgame. A
  third accident would leave the set short, and the shortfall is then researched with
  science.
- **What it saves.** Nanolathing costs 550 science and Experimental Motors 1,000. If the
  set holds the five 1,000-science nodes and eight 550-science nodes, it saves up to
  9,400 science that would otherwise be spent rather than converted to funds.
- **What the test must settle.** Whether the grant is random, whether it draws only on
  nodes whose prerequisites are met, and the discovery distance. If grants draw only on
  nodes whose prerequisites are met, the reserved set holds only nodes whose
  prerequisites are researched by then.

C13's visits to the monoliths fall in the same endgame.

**The test is deferred.** *2026-11-14.* Finding a monolith needs KerbNet's in-game
window, which kRPC cannot drive, so the test needs the operator's hands, and the
operator's attention is to be kept for what needs it. The answers are needed only by
the endgame, so task T14 of the first milestone waits until the test can run without the
operator, or until the endgame.

## The EVA jetpack as a margin

The operator noted on 2026-11-14 that the EVA jetpack has significant Δv, and asked for
an analysis, perhaps keeping it in reserve as a safety margin.

**What is established.**
- *(files)* The EVA Jetpack is a start-tech cargo part of 0.02 t. It holds 5 units of EVA
  Propellant, at 0.005 t per unit.
- *(web, the official wiki's EVA propellant and EVA Jetpack pages)* That is about 600 m/s
  of Δv for a kerbal. A modder's comparison gives 674 m/s. Since 1.11 a kerbal can
  carry up to 8 units with an extra tank, for which no Δv figure was found.
- *Computed from the running game's radii and gravity.* The table gives each airless
  body's ideal speed for an orbit at the surface and for escape, with no gravity losses,
  terrain or rotation.

| Body | Orbit, m/s | Escape, m/s | 600 m/s reaches |
|---|---|---|---|
| Gilly | 25 | 36 | Escape |
| Pol | 128 | 181 | Escape |
| Minmus | 172 | 243 | Escape |
| Bop | 196 | 277 | Escape |
| Ike | 378 | 535 | Escape |
| Dres | 395 | 558 | Escape |
| Mun | 571 | 807 | Orbit on paper only, since real losses exceed the margin |
| Eeloo | 595 | 842 | Orbit on paper only, as for the Mun |

**What follows.** *Inference.*
- **An abort mode on six bodies.** On Gilly, Pol, Minmus, Bop, Ike and Dres, a crew
  stranded on the surface could reach orbit on their jetpacks for a pickup. That gives
  R5 an abort mode that costs no extra hardware.
- **Biome hopping.** On the smallest bodies a kerbal could move between biomes by
  jetpack. That would save landers, if the jetpack refills from the lander's
  monopropellant on boarding *(recall, to test)*.
- **Crew transfer in orbit,** between vessels close enough together.

**Recommendation.** *Proposed.* The jetpack's Δv is held in reserve, and R6's budgets do
not count it. A mission plan may draw on it nominally only where its analysis of return
on investment shows the gain, such as biome hopping on a small body, and then keeps a
stated reserve for the abort. Phase 1 measures the jetpack's real Δv and thrust, and
whether it refills, before any plan relies on it.

## Rules for buying
## Rules for buying

1. **Research freely, buy late.** Research costs only science, which is plentiful.
   Purchases cost funds, which are not. A part is bought when a mission design needs
   it, and the parts no mission needs wait for the C3 campaign late in the game.
2. **Design for recovery.** The pad and the runway refund in full, and the area near
   the Space Center 98 percent *(web)*.
3. **Check on the pad.** Every career vehicle is rolled out and checked on the pad
   before flight, which costs nothing.
4. **Take only completable contracts.** A contract is accepted only when the current
   fleet can complete it, the world-first contracts before their firsts, and none is
   failed, since Hard doubles the penalty.

## After the window

*Sketch, to be detailed at E5.* With Research and Development at level 2, the 160 and
300 tiers open.
- **Remote control and science first.** Precision Engineering for the HECS core and
  the RA-2 relay, Advanced Exploration for the laboratory, and Actuators for the
  Advanced Grabbing Unit, which makes rescues possible without anyone leaving a vessel.
  Then Scanning Tech, Electronics, Field Science and Unmanned Tech.
- **A second pass of the Mun and Minmus.** Surface samples and reports outside the
  vessel add about 91 science per Mun biome and 114 per Minmus biome on Hard, about
  2,580 for both moons.
- **The laboratory near Minmus,** and the first interplanetary windows. The wiki's
  multipliers make every later body worth several times the Mun per subject.
- **Research and Development level 3,** at 2,535,000 funds, opens the 550 and 1,000
  tiers. That is the second funds gate of the campaign, met by stations, rescues and
  contracts beyond Kerbin. Surplus science stays banked until the tree is complete.
- **After the tree.** Once every node is researched, the operator expects the rest to be
  a matter of carrying out the work rather than of lining up prerequisites. *This
  document's assessment.* That is largely right, with funds as the exception. Buying
  every part costs 4,655,650 funds and the remaining facility levels several million
  more, and Patents Licensing may then turn surplus science into funds. Crew
  experience, launch windows and comets that appear at random set the pace in time
  rather than in prerequisites.

## Checkpoints

| Checkpoint | Measure | Decide |
|---|---|---|
| End of E0 | World-first payouts on Hard, the contracts on offer, whether a kerbal may leave a vessel in orbit at the first Astronaut Complex level, the level a scientist needs to reset experiments, and how kRPC stores several results in one flight | Whether rescues can start before E4, and the size of the gap |
| End of E3 | Funds earned per flight against this plan | The order of the launch pad and the assembly building upgrades, since 30 parts may bind before 18 tonnes |
| During E5 | Funds earned per contract, per world first and per recovered booster | When to buy Research and Development level 2 |
