# Campaign Plan

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Final, 2026-11-14. Every criterion and rule is decided by the operator.
Facts marked *(recall)* remain to be checked in Phase 0. Nothing has been flown. Each
fact names its source.

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

The campaign is complete when every criterion below is met. Criteria C1 to C7 restate
the operator's list. Criteria C8 to C18 were proposed by this document and adopted by
the operator on 2026-11-14.

| ID | Criterion | Game tracks it | Measured from | Standing |
|---|---|---|---|---|
| C1 | Research all 63 technology nodes, 18,468 science in total *(files)* | Yes | Save file, research state | Decided |
| C2 | Purchase all 461 researchable parts *(files)* | Yes | Save file, purchased parts | Decided |
| C3 | Fly every part in a role it was designed for | No | Ikria part ledger, from telemetry | Decided |
| C4 | Bring every science subject to its cap, with one sample per asteroid class and per comet class | Yes, per subject | Save file, science subjects | Decided |
| C5 | Complete every finite contract, at least one contract of every type at every body that offers it, and every stock mission in Mission mode | Partly | Save file and Mission mode results | Decided |
| C6 | Every Enhanced Edition trophy, and as a supplemental best effort, everything Kerbal Space Program 2 tracks that Kerbal Space Program 1 could have tracked | No | Achievement map in the appendix | Decided |
| C7 | No kerbal dies or goes missing, including tourists and rescued kerbals | Yes | Save file, crew roster | Decided |
| C8 | Every world first on every body where it can exist | Yes | Save file, progress tracking | Decided |
| C9 | Every progress record, such as altitude, depth, speed and distance | Yes | Save file, progress tracking | Decided |
| C10 | Every facility at its highest level | Yes | Save file, facility levels | Decided |
| C11 | At least two pilots, two engineers and two scientists at the highest rank | Yes, per kerbal | Save file, crew roster | Decided |
| C12 | A crewed landing and return on every body with a surface | Yes, as world firsts | Save file, progress tracking | Decided |
| C13 | Every anomaly visited, including the Green Monolith on each body | Partly | Save file and Ikria ledger | Decided |
| C14 | A resource survey from orbit of every body | Partly | Save file and Ikria ledger | Decided |
| C15 | An asteroid captured, and a sample from every comet class that appears | Partly | Save file and Ikria ledger | Decided |
| C16 | Every launch site flown from, including the Making History sites | No | Ikria ledger | Decided |
| C17 | Every strategy in the Administration Building run once | No | Ikria ledger | Decided |
| C18 | Every stock mission in Mission mode and every stand-alone scenario completed once without an incident | Not in career | Results and incident reports | Decided |

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
  them. Phase 0 enumerates the subjects from the running game. If asteroid and comet
  samples are separate subjects per object, the operator's bound of one sample per
  class keeps C4 finite. Science generated by the laboratory has no end, so it is
  excluded from C4.
- **C5.** The finite contracts are the story-line and world-first contracts *(web)*.
  The contract types are listed in the appendix. Mission mode is separate from career,
  and the installed game holds 9 Making History missions, 3 tutorials and the European
  Space Agency missions *(files)*. Each runs on its own settings, so R1 does not apply
  to it.
- **C6.** The rule is the operator's, given on 2026-11-14. Kerbal Space Program 2
  tracked 41 fixed missions, 41 points of interest that it called discoverables, its own
  science and its own technology tree *(web)*. Its kerbals had no ranks and it kept no
  record of firsts *(web, recalled by the source)*. The science, the tree and the
  discoverables fall under C1, C4 and C13. The missions are mapped one by one in the
  appendix. Where a mission names a place that exists only in Kerbal Space Program 2,
  Kerbal Space Program 1 could not have tracked it, so the place is dropped and the rest
  of the objective is still flown. Several missions add tasks that no other criterion
  requires. These are a landing of more than 200 tonnes on Minmus and of more than 300
  tonnes on Duna, a landing on Ike with three crew, a landing of ten kerbals on Eve with
  their return, a jet engine run on Laythe, and a fuel depot in orbit of Kerbin. The
  operator ruled on 2026-11-14 that everything taken from Kerbal Space Program 2 is a
  supplemental best effort, so these tasks do not hold back completion. The Enhanced
  Edition trophies are also mapped in the appendix, and each falls under another
  criterion.
- **C18.** Mission mode holds 9 Making History missions, 3 tutorials and the European
  Space Agency missions, and the game also ships 16 stand-alone scenarios *(files)*.
  Each attempt is played as one shot, with no reload inside it even where the mission
  allows one. An attempt that loses a kerbal, or that needs anything else R11 reports,
  still counts as a completion with incidents. The attempts continue until one completes
  with no incident report, and only that attempt meets C18. A scenario that states no
  objective of its own is complete when the aim its description gives is met.
- **C7.** Rescued kerbals and tourists count, because their deaths are deaths in the
  game's roster. A loss is handled under R10.
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

- **R1, difficulty.** *Decided.* The campaign plays the Hard preset unmodified.
  The game marks a save as Custom once any setting differs from a preset, so the highest
  standard difficulty means Hard exactly as shipped. Hard disables reverting flights
  and quickloading *(web)*, so the game offers no way back. R10 sets when a save is
  restored from outside the game. The preset's settings are in the appendix.
- **R2, capability parity.** *Decided.* Keleusma control loops send only the inputs a
  player could send at the career's current state. Ikria does not call any kRPC
  function that does something the career has not yet unlocked. Examples are placing
  manoeuvre nodes before the facilities allow them, and using a stability assist mode
  that the crew or the probe core cannot hold. Guidance and attitude control are written
  in Keleusma, so kRPC's built-in autopilot is not used. kRPC may not enforce these
  gates itself *(recall)*, which is why the harness must.
- **R3, signal.** *Decided.* Hard does not require a signal to control a probe *(web)*.
  The harness requires one anyway, as an agency rule, and refuses to command a probe
  that has no connection. The record should not depend on control that the game's own
  signal model says is unavailable.
- **R4, qualification in the sandbox.** *Decided.* Every vehicle flies its full
  mission profile in the sandbox save first, with the same craft file and the same
  Keleusma programs. This includes at least one abort case per flight phase. The
  sandbox's settings are matched to the career's where the game allows, such as
  reentry heating, the part and kerbal acceleration limits, pressure limits and the
  signal settings. The vehicle uses only parts the career holds, as the charter
  requires.
- **R5, crew safety.** *Decided.* No kerbal flies on a vehicle's first flight. Every
  crewed vehicle has an abort mode for every flight phase. Every first crewed landing on
  a body follows an uncrewed landing there that used the same descent and ascent
  profile. A crewed mission beyond Kerbin's sphere of influence flies only when a
  recovery option exists for its likeliest failure.
- **R6, margins.** *Decided.* Every mission states its budget for the change in
  velocity, written Δv, with a margin of at least 10 percent. It also budgets electric
  charge, monopropellant and the propellant for extravehicular activity, each with a
  stated margin. The 10 percent figure is a starting proposal, to be tuned by
  measurement.
- **R7, configuration record.** *Decided.* Every flight records the game version and
  settings, and the hash of every craft file and every Keleusma program it flew. It
  also records the Ikria commit and the save state before launch.
- **R8, mission record.** *Decided.* Every mission has a design report before
  flight, a telemetry log during flight, and a post-flight report after it. The design
  report covers objectives, trajectory, budgets, abort modes and failure modes. The
  post-flight report covers results and anomalies, and each anomaly gets an entry in a
  campaign anomaly log.
- **R9, information parity.** *Decided.* The operator ruled that a control loop may
  know what a craft could reasonably know, and accepted the following details. Knowledge
  sits in two places, and each is limited differently.
  - **On board.** A Keleusma program flying a vessel sees that vessel's full state,
    which stands in for its own sensors and navigation. This covers position, velocity,
    attitude, resources and the state of its parts.
  - **On the ground.** The mission controller sees an uncrewed vessel's state only
    while the vessel has a link. It sees other vessels only once the agency tracks them,
    and asteroids and comets only once they are discovered.
  - **Bodies.** The motion and physical constants of every body are known from the
    start.
  - **Terrain, biomes and anomalies.** Terrain height and biome at an arbitrary point
    are known for a body only after a survey of it from orbit. Before that, a vessel
    knows only what its own position and altitude above the ground give. Anomalies are
    known only once a scan reveals them.
  - **Enforcement.** The harness enforces these limits, and R2 and R3, outside the game.
    No mod that changes play is adopted, because the career is to stay Hard unmodified.
    kRPC already reports more than the limits allow, so the gap is closed by filtering
    what it reports, not by changing the game. A small read-only kRPC service is written
    only if Phase 0 finds state that kRPC does not expose and the save file cannot give
    in time. Candidates are facility levels, the tracking state of asteroids and comets,
    survey coverage, and anomalies revealed by a scan *(recall)*. Such a service would
    belong in the separate bridge repository of charter item 6, since kRPC is GPL-3.0.
- **R10, losses and reloading.** *Decided.* Hard leaves no reload inside the game. A
  save can still be restored from outside it, since the game keeps up to five rolling
  backups in each save's folder *(files)*, and the harness can keep its own copies. The
  operator ruled that a loss caused by a defect in the game is a reload scenario where
  reloading is possible, and that otherwise a loss is documented and the campaign
  continues. The operator accepted the following details.
  - **Loss from a game defect.** The save is restored from the harness's copy taken
    before the affected flight. The event goes into the anomaly log, with the evidence
    that it was a defect.
  - **Loss from a design or control fault.** The loss stands. It is documented, and
    the campaign continues.
  - **Classification.** The operator confirms whether a loss was a defect before any
    restore, so that the record cannot quietly undo its own mistakes.
- **R11, incident reports.** *Decided.* Every reload of the career save and every loss
  of a kerbal requires an incident report, as does every Mission mode or scenario
  attempt that loses a kerbal. The report gives the timeline from telemetry, the
  classification under R10, the cause and contributing factors, the corrective action,
  and the criteria affected. Each report is also an entry in the anomaly log of R8.
  The operator ruled on 2026-11-14 that the report is to follow space agency and
  aerospace industry standards, researched and published by the blog line. The request
  was sent that day, and the template follows the article. Until then the contents
  above stand. Nothing waits on it before the first incident report.
- **R12, operator attention.** *Decided.* The operator wants the campaign to need as
  little attention as possible. The line therefore works without asking, and reports
  through the record. The operator is needed only for three things. These are confirming
  a defect before a restore under R10, the outward actions the repository rules reserve,
  such as merging, and changes to these criteria and rules. The operator granted a
  standing authority on 2026-11-14 to push routine documentation and record pull
  requests, which `CLAUDE.md` defines. Requests are batched where the work allows,
  such as one merge per milestone.

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
| 0, ground work | Build the bridge, the harness and the records of R7 and R8. Create the career save "Ikria Automated Space Agency" on Hard and the sandbox save "Ikria Simulated Research and Development", and record the Hard preset's full settings from the new career save. Match the sandbox settings to the career's. Enumerate bodies, biomes, situations, experiments, parts, progress nodes and facility gates from the running game, as the baseline of the completion ledger. | The ledger lists every subject, part and progress node, and the harness flies a qualification profile in the sandbox. |
| 1, Kerbin | Science at the Space Center's biomes and in Kerbin's situations. Flights short of orbit, the first orbit, the first extravehicular activity, the record contracts, the first upgrades. | Crewed orbit and return, and Kerbin's near-space subjects collected. |
| 2, Mun and Minmus | Uncrewed flyby, orbit and landing, then crewed landing and return on both. Begin biome-by-biome science. | Crewed returns from both, and the 160 tier of the tree within reach. |
| 3, infrastructure | A relay network around Kerbin, a station with a science laboratory, docking, a fuel depot and refuelling in orbit, and resource extraction on Minmus. | Refuelling demonstrated, and the laboratory in operation. |
| 4, inner planets | Duna and Ike, Eve and Gilly, Moho, each with relays, uncrewed precursors, then crewed landings and returns. | Crewed returns from Duna, Ike, Gilly and Moho. The crewed return from Eve may move to Phase 6. |
| 5, outer planets | Dres, Jool and its five moons, and Eeloo, on the same pattern. | Crewed returns from every outer body with a surface. The return from Tylo may move to Phase 6. |
| 6, completion | Remaining biomes by rover, hopper and aircraft. The heavy landings of C6, including ten kerbals on Eve and back. Anomalies, asteroids and comets, remaining contract types, unflown parts, crew ranks, records and the achievement map. | Every adopted criterion met. |

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

## Rulings, 2026-11-14

The operator answered the first eight questions on 2026-11-14.

| Question | Ruling |
|---|---|
| Q1, difficulty and reloading | Hard unmodified. Charter item 2 is amended so that quickloads and reverts apply to the sandbox. The operator asked whether saves can still be reloaded, which R10 answers. |
| Q2, missions | Both readings, the finite and typed contracts of career and every stock mission in Mission mode. See C5 and C18. |
| Q3, the bound on science | One sample per asteroid class and per comet class. See C4. |
| Q4, information parity | Yes, to the extent that it would be possible and reasonable for a craft. See R9. |
| Q5, defects in the game | A reload scenario if reloading is possible, and otherwise documented and continued. See R10. |
| Q6, achievements | If Kerbal Space Program 2 tracks something that Kerbal Space Program 1 could have tracked, it is to be achieved. See C6. |
| Q7, the proposals | All additional criteria and rules accepted. |
| Q8, saves | A new career save on Hard, with a new companion sandbox save. The existing saves are left untouched. |

## Rulings, second set, 2026-11-14

| Question | Ruling |
|---|---|
| O1, information details | Accepted. The operator asked whether a mod should do some of it, which R9's enforcement note answers. |
| O2, reload details | Accepted. Every reload or loss of a kerbal in career needs an incident report, now R11. The campaign should need as little of the operator's attention as possible, now R12. |
| O3, the Kerbal Space Program 2 map | Understood. Everything taken from Kerbal Space Program 2 is a supplemental best effort. |
| O4, Mission mode | No-death completions, with one-shot attempts repeated until one completes without an incident report. See C18. |
| O5, save names | "Ikria Automated Space Agency" for the career save and "Ikria Simulated Research and Development" for the sandbox save. |
| O6, training and scenarios | Stand-alone scenarios are in scope, as for Mission mode. The operator has mixed feelings on the training lessons. |

## Rulings, third set, 2026-11-14

| Question | Ruling |
|---|---|
| O7, training lessons | Omitted. |
| O8, standing authority to push | Granted, and recorded in `CLAUDE.md`. |

No questions remain open.

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

### Kerbal Space Program 2 missions

From the community wiki page [Missions](https://kerbal.wiki/Missions) and the game's
0.2.0.0 release notes on Steam, read on 2026-11-14 *(web)*. The game's last version was
0.2.2.0, of 2024-06-11. It had 11 primary and 30 secondary missions, each with a name
and a reward in science, which the wiki page gives. This table lists them by objective.
Version 0.2.2.0 added a landing or splashdown to the first two primary missions. Values
marked *(recall)* in the mapping column are still to be checked in Kerbal Space Program
1.

| ID | Objective in Kerbal Space Program 2 | Task in Kerbal Space Program 1 |
|---|---|---|
| P1 | Reach 10 km, then land or splash down | The same, under C8 and C9 |
| P2 | Rise above 70 km, then land or splash down | The same, under C8 |
| P3 | Orbit Kerbin with periapsis above 70 km and apoapsis below 300 km | The same |
| P4 | Orbit the Mun with periapsis above 60 km and apoapsis below 2,400 km | The same |
| P5 | Land on the Mun | C12 |
| P6 | Find a signal source on the Mun | Visit a Mun monolith, under C13 |
| P7 | Put a probe with an antenna in orbit of Minmus | The same |
| P8 | Reach the monument on Minmus | Visit the Minmus monolith, under C13 |
| P9 | Reach the monument on Duna | Visit the Duna pyramid, which emits a signal, under C13 |
| P10 | Put a probe in orbit of Jool with an antenna of at least 86 Gm range | The same, with a relay antenna of at least that rating |
| P11 | Reach the monument on Tylo | Visit the Tylo cave, under C13 |
| S1 | Land in water on Kerbin | C8 |
| S2 | Walk in space in orbit of Kerbin | C8 |
| S3 | Run an environment survey with the small materials experiment | Run the materials bay in flight over Kerbin, under C4 |
| S4 | Orbit Kerbin with periapsis and apoapsis both between 99 and 101 km | The same |
| S5 | Orbit Kerbin with periapsis 70 to 150 km and apoapsis 300 to 8,400 km | The same |
| S6 | Put a probe with an antenna and a solar panel in orbit of Kerbin | The same |
| S7 | Launch a vessel with at least four wheels | The same, under C3 |
| S8 | Crew observation with a lander can at Kapy Rock on Kerbin | A crew report from a landed lander can on Kerbin, since the place is absent |
| S9 | Crew observation with a lander can at Stargazer Point on Kerbin | As S8 |
| S10 | Keostationary orbit, 2,863 km at 1,010 m/s | The same, which is also Kerbin's synchronous orbit *(recall)* |
| S11 | Put the largest standard fuel tank and a docking port in orbit | The same, with the Rockomax Jumbo-64 *(recall)* |
| S12 | Plant a flag in a mare on the Mun | Plant a flag in a lowland biome of the Mun *(recall)* |
| S13 | Land more than 200 t on Minmus | The same |
| S14 | Leave Kerbin's sphere of influence | C8 |
| S15 | Land on Ike with three crew | The same |
| S16 | Land more than 300 t on Duna | The same |
| S17 | Enter Eve's sphere of influence | C8 |
| S18 | Land ten kerbals on Eve and return them | The same, the largest single crewed task found |
| S19 | Environment survey at the Oblique Impact site on Gilly | A materials bay landed on Gilly, since the place is absent |
| S20 | Land in the Mohole on Moho | The same, under C13 |
| S21 | Return a surface sample from the Croissant on Moho | A surface sample returned from Moho, since the place is absent |
| S22 | Atmosphere survey at Jool | Atmosphere analysis in Jool's atmosphere, under C4 |
| S23 | Plant a flag on Pol | C8 |
| S24 | Radiation observation in Pol's lowlands | No counterpart, since Kerbal Space Program 1 has no radiation experiment *(recall)* |
| S25 | Sample the creature on Laythe | Visit the Laythe skeleton and take a surface sample there |
| S26 | Sample the fossil on Bop | Visit the dead kraken on Bop and take a surface sample there |
| S27 | Run a jet engine on Laythe | The same |
| S28 | Plant a flag in the Eye of Dres | Plant a flag on Dres, since the place is absent |
| S29 | Enter Eeloo's sphere of influence | C8 |
| S30 | Drive a rover onto the Frozen Tsunami on Eeloo | Drive a rover on Eeloo, since the place is absent |

The 41 discoverables of Kerbal Space Program 2 are its own places, so C13 is met with
Kerbal Space Program 1's anomalies instead.

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
