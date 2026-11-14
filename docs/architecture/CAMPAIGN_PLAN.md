# Campaign Plan

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Proposed, 2026-11-14. Nothing has been flown. Each point below is marked
**Decided**, **Proposed** or **Open**, and each fact names its source.

This document plans the career campaign of Kerbal Space Program 1 that Ikria's
Keleusma control loops are to fly. It defines what complete means, the rules the
campaign keeps, the constraints that order it, and the phases that follow from them. It
does not yet fix individual missions. Those belong in the roadmap, one milestone at a
time, once the operator settles the open questions at the end.

## Operator brief, 2026-11-14

**Decided.** The goal is to complete everything that career mode tracks, on the highest
standard difficulty, and anything that ought to be tracked even if the game does not
track it. The operator named the following.

1. Unlock all parts.
2. Fly all parts in sensible configurations.
3. Collect all science points, even when no more science is needed to unlock anything.
4. Complete all missions.
5. Complete every task equivalent to a Kerbal Space Program 2 achievement.
6. No kerbal dies.

The operator expects that this list misses something. The campaign is to be run as a
very hardcore space agency simulation, so that the solution and its sequencing are
useful to professionals who are willing and able to digest it. This document reads that
as a standard for the record. A reader with professional mission-planning training
should be able to audit every decision from the record alone.

## Sources

- **Game files.** Counts marked *(files)* were computed on 2026-11-14 by a read-only
  script over `GameData/Squad` and `GameData/SquadExpansion` of the installed game,
  KSP 1.12.5 with both expansions. Nothing was copied from them.
- **Web.** Facts marked *(web)* were read on 2026-11-14, mostly from the official wiki
  through its public interface. Several community sites refused automated reading, so
  some facts have one source only. The appendices name the pages.
- **Recall.** Facts marked *(recall)* come from memory of the game and have not been
  checked. Each is to be verified in game before a decision depends on it.

## Completion criteria

The campaign is complete when every adopted criterion below is met. Criteria C1 to C7
restate the operator's list. Criteria C8 onward are this document's proposals for what
ought to be tracked, and each awaits the operator's adoption.

| ID | Criterion | Game tracks it | Measured from | Standing |
|---|---|---|---|---|
| C1 | Research all 63 technology nodes, 18,468 science in total *(files)* | Yes | Save file, research state | Decided |
| C2 | Purchase all 461 researchable parts *(files)* | Yes | Save file, purchased parts | Decided |
| C3 | Fly every part in a role it was designed for | No | Ikria part ledger, from telemetry | Proposed definition |
| C4 | Bring every science subject to its cap | Yes, per subject | Save file, science subjects | Open bound, see Q3 |
| C5 | Complete all missions | Partly | Depends on Q2 | Open, see Q2 |
| C6 | Complete every task equivalent to an achievement | No | Achievement map in the appendix | Open, see Q6 |
| C7 | No kerbal dies or goes missing, including tourists and rescued kerbals | Yes | Save file, crew roster | Decided |
| C8 | Every world first on every body where it can exist | Yes | Save file, progress tracking | Proposed |
| C9 | Every progress record, such as altitude, depth, speed and distance | Yes | Save file, progress tracking | Proposed |
| C10 | Every facility at its highest level | Yes | Save file, facility levels | Proposed |
| C11 | At least two pilots, two engineers and two scientists at the highest rank | Yes, per kerbal | Save file, crew roster | Proposed |
| C12 | A crewed landing and return on every body with a surface | Yes, as world firsts | Save file, progress tracking | Proposed |
| C13 | Every anomaly visited, including the Green Monolith on each body | Partly | Save file and Ikria ledger | Proposed |
| C14 | A resource survey from orbit of every body | Partly | Save file and Ikria ledger | Proposed |
| C15 | An asteroid captured, and a sample from every comet class that appears | Partly | Save file and Ikria ledger | Proposed |
| C16 | Every launch site flown from, including the Making History sites | No | Ikria ledger | Proposed |
| C17 | Every strategy in the Administration Building run once | No | Ikria ledger | Proposed, optional |
| C18 | Every stock Making History mission completed in Mission mode | Not in career | Mission mode results | Depends on Q2 |

**Notes on the criteria.**
- **C1 and C2.** The game defines 489 part blocks *(files)*. Of these, 2 cannot be
  researched, since they are the asteroid and comet bodies themselves. Another 11 are
  hidden older versions of current parts, kept so that old craft still load. That
  leaves 461 parts, 340 from the base game, 69 from Making History and 52 from Breaking
  Ground. Two nodes, `experimentalMotors` and `nanolathing`, hold no listed part, but
  they still count toward C1.
- **C3.** The proposed definition is that each part is flown at least once with its
  function exercised in the situation it was built for. An engine fires in flight. A
  parachute opens during a descent. A docking port docks. A heat shield takes a
  reentry. A wheel drives. A robotic part actuates under load. An experiment returns or
  transmits data. A sensible configuration is one that the mission's design report
  justifies on engineering grounds, so no part is carried only to tick a box.
- **C4.** The game holds 56 experiment definitions *(files)*. Of these, 18 come from
  the base game, including the asteroid and comet samples. Another 4 are Breaking
  Ground deployed experiments. The last 34 are Breaking Ground surface features, which
  their names place on 15 bodies. A science subject is one experiment in one situation
  at one body, and for most experiments in one biome. The full list of subjects cannot
  be read from the files, because the body multipliers and the biome maps are not in
  them. Phase 0 enumerates the subjects from the running game.
- **C7.** Rescued kerbals and tourists count, because their deaths are deaths in the
  game's roster.
- **C8.** The save's progress tracking holds firsts such as first launch, reaching
  space and the first crew to survive. For each body it holds flyby, orbit, landing,
  splashdown, escape, return from flyby, return from orbit, return from the surface,
  rendezvous, docking, and extravehicular activity in space and on the surface *(web)*.
  Flag planting, crew transfer, station and base construction, and anomaly discovery
  may also be tracked *(recall)*. The exact list is read from a save in Phase 0. Some
  firsts cannot exist. The Sun and Jool have no surface, and only some bodies have
  oceans.
- **C12.** This includes crewed landings and returns on Eve and Tylo, which are
  expected to need the heaviest vehicles of the campaign *(recall)*.
- **C13.** The anomalies are listed in the appendix *(web)*. The Green Monolith appears
  once on each of 15 bodies, at a place chosen per save, and is found through KerbNet,
  the map that some probe cores and scanners provide *(web)*. Visited means a kerbal
  stands at the anomaly. For an anomaly no kerbal can reach, such as the one on Kerbin's
  seabed, a vessel within sight of it counts.

## Rules of play

- **R1, difficulty.** *Open, see Q1.* The campaign plays the Hard preset unmodified.
  The game marks a save as Custom once any setting differs from a preset, so the highest
  standard difficulty means Hard exactly as shipped. Hard disables reverting flights
  and quickloading *(web)*, so every loss is final. The preset's settings are in the
  appendix.
- **R2, capability parity.** *Proposed.* Keleusma control loops send only the inputs a
  player could send at the career's current state. Ikria does not call any kRPC
  function that does something the career has not yet unlocked. Examples are placing
  manoeuvre nodes before the facilities allow them, and using a stability assist mode
  that the crew or the probe core cannot hold. Guidance and attitude control are written
  in Keleusma, so kRPC's built-in autopilot is not used. kRPC may not enforce these
  gates itself *(recall)*, which is why the harness must.
- **R3, signal.** *Proposed.* Hard does not require a signal to control a probe *(web)*.
  This document proposes that the harness require one anyway, as an agency rule, and
  refuse to command a probe that has no connection. The record should not depend on
  control that the game's own signal model says is unavailable.
- **R4, qualification in the sandbox.** *Proposed.* Every vehicle flies its full
  mission profile in the sandbox save first, with the same craft file and the same
  Keleusma programs. This includes at least one abort case per flight phase. The
  sandbox's settings are matched to the career's where the game allows, such as
  reentry heating, the part and kerbal acceleration limits, pressure limits and the
  signal settings. The vehicle uses only parts the career holds, as the charter
  requires.
- **R5, crew safety.** *Proposed.* No kerbal flies on a vehicle's first flight. Every
  crewed vehicle has an abort mode for every flight phase. Every first crewed landing on
  a body follows an uncrewed landing there that used the same descent and ascent
  profile. A crewed mission beyond Kerbin's sphere of influence flies only when a
  recovery option exists for its likeliest failure.
- **R6, margins.** *Proposed.* Every mission states its budget for the change in
  velocity, written Δv, with a margin of at least 10 percent. It also budgets electric
  charge, monopropellant and the propellant for extravehicular activity, each with a
  stated margin. The 10 percent figure is a starting proposal, to be tuned by
  measurement.
- **R7, configuration record.** *Proposed.* Every flight records the game version and
  settings, and the hash of every craft file and every Keleusma program it flew. It
  also records the Ikria commit and the save state before launch.
- **R8, mission record.** *Proposed.* Every mission has a design report before
  flight, a telemetry log during flight, and a post-flight report after it. The design
  report covers objectives, trajectory, budgets, abort modes and failure modes. The
  post-flight report covers results and anomalies, and each anomaly gets an entry in a
  campaign anomaly log.

## Constraints that order the campaign

1. **Science gates parts.** *(files)* The tree's node costs rise in tiers. The table
   gives the cumulative science needed to finish each tier. Hard pays 60 percent of the
   normal science reward *(web)*, which lengthens every step.

   | Node cost | Nodes | Cumulative science |
   |---|---|---|
   | 5 to 20 | 5 | 63 |
   | 45 | 5 | 288 |
   | 90 | 10 | 1,188 |
   | 160 | 13 | 3,268 |
   | 300 | 12 | 6,868 |
   | 550 | 12 | 13,468 |
   | 1,000 | 5 | 18,468 |

2. **Parts gate destinations.** Each destination needs a Δv, a heat tolerance, a
   landing mass and a signal path that only some parts provide. The tree finishes long
   before the science runs out, so the late campaign is limited by vehicles and launch
   windows, not by science.
3. **Funds gate everything that is built.** Hard starts with 10,000 funds, pays 60
   percent of the normal funds reward, and doubles funds and reputation penalties
   *(web)*. It also requires each part to be purchased after its node is researched
   *(web)*, and the sum of all part entry costs is 4,655,650 funds *(files)*. Funds pay
   for launches, recruits, part purchases and facility upgrades, and come from
   contracts, world-first milestones and recovered hardware. Funds are therefore the
   binding constraint early, and a part is bought only when a mission needs it.
4. **Facilities gate capabilities.** *(recall)* Facility levels limit part count, mass
   and size at the Vehicle Assembly Building, the Spaceplane Hangar, the launch pad and
   the runway. The Tracking Station's level sets whether orbit patches are shown and the
   power of the ground network. Mission Control's level sets whether manoeuvre nodes are
   available and how many contracts may be active. The Astronaut Complex's level sets
   whether kerbals may leave a vessel away from Kerbin and how many may be hired. The
   Research and Development facility's level sets whether surface samples may be taken.
   The Administration Building's level sets how many strategies may run. Each gate is to
   be confirmed in game in Phase 0.
5. **Crew is finite.** Hard neither hires crew automatically nor returns missing crew
   *(web)*. Crew experience gates skills *(recall)*. Pilots unlock stability assist modes
   by level. Engineers repair parts and improve drills and converters. Scientists
   improve science yield and reset experiments. Kerbals gain experience by visiting
   bodies and recovering on Kerbin, so crew training is itself a sequenced activity.
6. **Signal shapes the network.** Hard scales the range of every antenna to 0.65 of
   normal and lets terrain and atmosphere block the signal fully *(web)*. Under R3 a
   relay network must precede every deep uncrewed mission.
7. **Launch windows gate departures.** Transfers to other planets open at intervals set
   by each pair of orbits. The calendar is computed from the orbit elements the game
   itself reports through kRPC, rather than from published tables. The order of the
   interplanetary phases follows that calendar.
8. **Science per subject rises with distance.** The bodies far from Kerbin pay more per
   subject. Early science therefore comes from the Space Center's own small biomes, from
   Kerbin's situations, and from the Mun and Minmus.

## Phases

The phases overlap. Contracts run throughout, and launch windows can open a later phase
before an earlier one is finished. Each phase ends when its exit condition is met. The
exit condition is what the roadmap's milestones will test.

| Phase | Work | Exit condition |
|---|---|---|
| 0, ground work | Build the bridge, the harness and the records of R7 and R8. Match the sandbox settings to the career's. Read the Hard preset's full settings from a fresh save. Enumerate bodies, biomes, situations, experiments, parts, progress nodes and facility gates from the running game, as the baseline of the completion ledger. | The ledger lists every subject, part and progress node, and the harness flies a qualification profile in the sandbox. |
| 1, Kerbin | Science at the Space Center's biomes and in Kerbin's situations. Flights short of orbit, the first orbit, the first extravehicular activity, the record contracts, the first upgrades. | Crewed orbit and return, and Kerbin's near-space subjects collected. |
| 2, Mun and Minmus | Uncrewed flyby, orbit and landing, then crewed landing and return on both. Begin biome-by-biome science. | Crewed returns from both, and the 160 tier of the tree within reach. |
| 3, infrastructure | A relay network around Kerbin, a station with a science laboratory, docking, refuelling in orbit, and resource extraction on Minmus. | Refuelling demonstrated, and the laboratory in operation. |
| 4, inner planets | Duna and Ike, Eve and Gilly, Moho, each with relays, uncrewed precursors, then crewed landings and returns. | Crewed returns from Duna, Ike, Gilly and Moho. The crewed return from Eve may move to Phase 6. |
| 5, outer planets | Dres, Jool and its five moons, and Eeloo, on the same pattern. | Crewed returns from every outer body with a surface. The return from Tylo may move to Phase 6. |
| 6, completion | Remaining biomes by rover, hopper and aircraft. Anomalies, asteroids and comets, remaining contract types, unflown parts, crew ranks, records and the achievement map. | Every adopted criterion met. |

## What the record produces

The campaign is published as a record for professional readers. It holds five kinds of
document.

- **Mission dossiers.** One for each mission, per R8.
- **Campaign ledger.** The standing of every adopted criterion, generated from the save
  file and from telemetry.
- **Budgets over time.** Science, funds and reputation, from the save file at each
  mission boundary.
- **Risk register and anomaly log.** What could go wrong next, and what did.
- **Program catalogue.** The vehicle-control routine pairs of charter item 10, with the
  sequencing that makes them work, each tied to the missions that flew it.

## Open questions

- **Q1, difficulty and reloading.** Charter item 2 allows quickloads and reverts. Hard
  unmodified disables both *(web)*, so the two conflict. Either the campaign plays Hard
  unmodified and every loss is final, or it enables reloading and the save is marked
  Custom. This document recommends Hard unmodified, with charter item 2 amended to
  apply to the sandbox only. The sandbox already absorbs testing, and an irreversible
  record is what makes C7 meaningful.
- **Q2, missions.** Career mode generates contracts without end, so all of them cannot
  be completed *(web)*. One reading is every finite contract, which means the story-line
  and world-first contracts, together with at least one contract of every type at every
  body where that type is offered. The types are listed in the appendix. Another reading
  is the stock missions of Making History, which run in a separate Mission mode and not
  in career. The installed game holds 9 such missions, 3 tutorials and the European
  Space Agency missions *(files)*. Which reading applies, or both?
- **Q3, the bound on science.** It is not yet established whether asteroid and comet
  samples are separate subjects per object. If they are, the subjects are unbounded and
  C4 needs a rule, such as one sample per class. Science from the laboratory is also
  generated without end, so it is excluded from C4. A comet class that never appears
  cannot be sampled, which C15 already allows for.
- **Q4, information parity.** R2 limits actions. The question is whether information is
  limited too. kRPC reports exact state that a player reads only through the
  instruments and the map. This document recommends allowing full state for vessels in
  flight. It also recommends hiding objects that the career has not yet discovered or
  tracked, such as unseen asteroids and comets.
- **Q5, defects in the game.** If a kerbal dies because of a defect in the game rather
  than a fault of design or control, is the campaign restarted, or is the loss recorded
  and the campaign continued? Under Hard unmodified, restoring an outside backup of the
  save would amount to a reload.
- **Q6, achievements.** Kerbal Space Program 2 appears to have shipped no Steam
  achievements *(web)*. Its Steam pages list none, and no other source could be read to
  confirm it. The nearest list is the 45 trophies of Kerbal Space Program 1 Enhanced
  Edition on consoles *(web)*, which the appendix maps onto the criteria. Is that the
  list the operator meant, or something else from Kerbal Space Program 2, such as its
  in-game missions?
- **Q7, which proposals to adopt.** C8 to C18, and R2 to R8.
- **Q8, the existing career save.** The game holds a career save named "Rookie" on the
  Normal preset *(files)*. This document assumes the campaign starts a new save and
  leaves that one untouched.

## Appendix, Hard preset

From the official wiki pages
[Game Difficulty](https://wiki.kerbalspaceprogram.com/wiki/Game_Difficulty) and
[CommNet](https://wiki.kerbalspaceprogram.com/wiki/CommNet), read on 2026-11-14. Both
pages predate some later settings.

| Setting | Normal | Hard |
|---|---|---|
| Allow reverting flights | Yes | No |
| Allow quickloading | Yes | No |
| Missing crews respawn | Yes | No |
| Auto-hire crew | Yes | No |
| No entry purchase required on research | Yes | No |
| Indestructible facilities | No | No |
| Allow stock vessels | No | No |
| Starting funds | 25,000 | 10,000 |
| Starting science and reputation | 0 | 0 |
| Funds, science and reputation rewards | 100% | 60% |
| Funds and reputation penalties | 100% | 200% |
| Require signal for control | Off | Off |
| Plasma blackout | Off | Off |
| Antenna range modifier | 1.00 | 0.65 |
| Occlusion in vacuum and in atmosphere | 0.90 and 0.75 | 1.00 and 1.00 |

Not found in any readable source, so to be read from a fresh Hard save in Phase 0. These
are kerbal experience, immediate level up, reentry heating, resource abundance, the
kerbal and part acceleration limits, part pressure limits, the building damage
multiplier, and the crossfeed rules. Whether quicksaving, as distinct from quickloading,
still works on Hard is also unverified.

## Appendix, achievement equivalents

Kerbal Space Program 1 Enhanced Edition has 45 trophies, 1 platinum, 1 gold, 18 silver
and 25 bronze *(web)*. The source is
[PlatPursuit](https://platpursuit.com/games/NPWR24410_00/wolfiemiguel/), and the Xbox
list on [Vandal](https://vandal.elespanol.com/logros/xbone/kerbal-space-program/32994)
holds the same 44 without the platinum. Neither source defines what "visited" requires,
so this map takes the strictest reading, a crewed landing and return.

| Trophies | Requirement as listed | Criterion |
|---|---|---|
| 15 "visited" trophies, one each for the Mun, Minmus, Duna, Ike, Eve, Gilly, Moho, Dres, Jool, Laythe, Tylo, Vall, Bop, Pol and Eeloo | Visit the body | C12, and C8 for Jool, which has no surface |
| 15 flyby trophies, including Kerbin, and the Eeloo flyby trophy | First flyby of the body | C8 |
| Look Ma, No Tractor Beams! | First docking | C8 |
| Here Comes the Kerbalry, and The Right Kind of Pilot | Recover the first crew, and reach space and land safely | C8 |
| Let Me Play Among the Stars | First uncrewed vessel in orbit, by its title | C8 or C5, to be confirmed |
| Failure is Not an Option | First Moho landing | C8 |
| Ground Control | First landing at the Space Center | C8 |
| Mun Rover, Mun Rover | First rover drive | C3 |
| Munwalking, and Zero Gravity | First extravehicular activity on the Mun, and in space | C8 |
| One Small Step | First flag on the Mun | C8 |
| No Left Turns, and Space, Space, SPAAAAACE! | First runway launch, and first launch pad launch | C16 |
| Open for business! | First contract | C5 |
| Master Astrophysicist | All others | All of the above |

## Appendix, anomalies

From the official wiki pages
[List of easter eggs](https://wiki.kerbalspaceprogram.com/wiki/List_of_easter_eggs) and
[Green Monolith](https://wiki.kerbalspaceprogram.com/wiki/Green_Monolith), read on
2026-11-14. Phase 0 confirms each location in game.

| Body | Fixed anomalies |
|---|---|
| Kerbin | Five monoliths, the inland second space center, the desert pyramid, the polar crashed vessel, the Island Airfield hangar, the seabed face, a crashed early probe, and an early liquid-fuel rocket |
| Mun | Three monoliths, three rock arches, one of which has changed in 1.12.5, a memorial, and a crashed vessel |
| Minmus | One monolith |
| Duna | A face, a camera mast, a pyramid, and a stranded rover |
| Ike | A boulder |
| Dres | A crashed car |
| Laythe | A skeleton and flowers |
| Vall | A henge, reported by the wiki to sit underground in some saves |
| Tylo | A cave |
| Bop | A dead kraken |
| Eeloo | A figure built from parts |
| Moho | A deep hole, with its own biome |
| Eve, Gilly, Pol and Jool | None listed |

The Green Monolith appears once on each body except Jool and the Sun, at a place chosen
per save *(web)*.

## Appendix, contract types

From the official wiki page [Contract](https://wiki.kerbalspaceprogram.com/wiki/Contract),
whose list was last checked against version 1.3, with later types from the game's own
change logs, read on 2026-11-14. The game's contract configuration names 19 groups
*(files)*. These are asteroid recovery, outpost construction, flag planting, grand tour,
resource extraction, world firsts, rescue and recovery, probe deployment to set orbits, science,
sentinel, comet detection, station construction, survey, part testing, tourism, comet
samples, rover construction, vessel repair, and construction in orbit. Breaking Ground
adds deployed science and surface feature contracts *(web)*. Making History appears to
add no career contracts *(web)*.
