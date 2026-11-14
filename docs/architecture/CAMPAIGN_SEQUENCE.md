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
  Mite sits in a 90-science node, inside the first research window, and the Sepratron I
  and the F3S0 Shrimp sit in 160-science nodes.
- **Pair the solids with control.** The more realistic design pairs small solid motors
  with reaction control and small engines that can be throttled. *(files)* The Ant and
  the Spark, and the RV-105 thruster block, are in 90-science nodes. The Spider, the
  Twitch and the Puff are in 160-science nodes.
- **Leave what earns its place.** A lander left on a surface is acceptable when an
  analysis of return on investment supports it. Fitted with solar panels, batteries and
  relay hardware, it becomes a surface relay or a historical base under rule R13 of the
  campaign plan. *Inference.* A left lander with the right equipment may also meet
  outpost contracts and the world first for a base.
- **Spent stages can be infrastructure.** A stage left in orbit with relay hardware, and
  serving as a relay, is not debris.

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
