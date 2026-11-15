# Astronaut Training Program

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Decided as a requirement by the operator on 2026-11-14. The program below is
proposed. Facts come from the official wiki's
[Experience](https://wiki.kerbalspaceprogram.com/wiki/Experience) page and its table of
experience by body, read on 2026-11-14 *(web)*, unless marked otherwise.

The operator asked for a kerbal astronaut training program, given how much there is to
do. Criterion C11 needs at least two pilots, two engineers and two scientists at the
highest rank, and many tasks need a trained kerbal on the spot. Keleusma programs fly
every vehicle, so training here means two things. One is the experience and rank the
game tracks. The other is the procedures a kerbal carries out outside the vessel, which
Keleusma programs also drive and which are qualified like any other program.

## How the game grants rank

- **Levels.** Rank runs from 0 to 5, at 2, 8, 16, 32 and 64 experience in total.
- **Earned once per body.** A kerbal earns experience once per body, for the best
  achievement there. Orbiting the Mun and then planting a flag there gives the flag's 5,
  not 8.
- **Banked on return.** Experience becomes rank only on recovery at the Space Center, or
  through the Mobile Processing Lab's Level Up Crew function.
- **Flags are shared.** Since 1.2, one planted flag grants its experience to every landed
  or splashed kerbal within physics range.
- **Sandbox gives none.** Experience is off in the sandbox, where every kerbal acts at the
  top level. Sandbox rehearsals train procedures but give no rank.

| Body | Flyby | Orbit | Flight | Landing | Flag |
|---|---|---|---|---|---|
| Kerbin | 1 | 2 | 1 | 0 | 0 |
| Mun | 2 | 3 | | 4.6 | 5 |
| Minmus | 2.5 | 3.75 | | 5.75 | 6.25 |
| Kerbol | 4 | 6 | | | |
| Duna and Ike | 5 | 7.5 | 10 for Duna | 11.5 | 12.5 |
| Eve | 5 | 7.5 | 10 | 11.5 | 12.5 |
| Moho | 7 | 10.5 | | 16.1 | 17.5 |
| Gilly and Dres | 6 | 9 | | 13.8 | 15 |
| Jool | 6 | 9 | 12 | | |
| Laythe, Vall, Tylo, Bop and Pol | 8 | 12 | 16 for Laythe | 18.4 | 20 |
| Eeloo | 10 | 15 | | 23 | 25 |

## What rank changes for this campaign

*From the wiki's specialist pages, read earlier on 2026-11-14 for the community practice
appendix of the campaign plan.*
- **Safety.** A kerbal other than a pilot can use the personal parachute only from level
  1. A pilot can use it at level 0. Any abort mode that relies on a kerbal leaving the
  vessel in the atmosphere therefore needs a pilot, or a crew of level 1 or higher. R5's
  abort analysis checks this for each crew.
- **Engineers.** Repairs and repacking parachutes need level 1. Drills and converters work
  at any level, with efficiency rising from 25 percent at level 0 to 85 percent at
  level 3.
- **Scientists.** The laboratory works with a scientist of any level, and its rate rises
  with level.
- **Pilots.** A pilot's stability assist modes do not matter to Keleusma programs, which
  fly through the raw controls under R2. Pilots matter for C11 and for the parachute rule
  above.

## The program

1. **Recruitment.** The corps grows mainly through rescue contracts, since the cost of
   each hire rises with the number already hired. Hiring is a fallback, priced in the
   funds plan.
2. **Basic, to level 1.** A suborbital flight and recovery, or an orbit of Kerbin, gives 1
   or 2 experience. That reaches level 1 at 2, which every kerbal needs before a crewed
   flight where bailing out could be an abort mode. *Proposed rule.* No kerbal other than
   a pilot flies a crewed mission beyond Kerbin's atmosphere at level 0.
3. **Lunar, to level 3.** A flag on the Mun, a flag on Minmus and an orbit of the Sun,
   recovered, give 17.25 experience and level 3. This is the community's standard loop.
   One mission can carry several kerbals, and a single flag serves everyone landed
   nearby.
4. **Interplanetary, to levels 4 and 5.** Level 5 needs 64 experience, which only flags on
   other planets reach. One path is the lunar loop's 17.25, Kerbin's orbit at 2, flags on
   Duna and Ike at 12.5 each, a flag on Dres at 15, and a flyby of Eve at 5, which make
   64.25. The six kerbals of C11 rotate through the interplanetary landings until each
   reaches level 5.
5. **Procedures.** Every task a kerbal does outside the vessel, such as a report, a sample,
   a flag, a repair or boarding, is a Keleusma program qualified in the sandbox under R4,
   as Phase 0's verification report VR-0003 began.
6. **Assignment.** Each mission plan names its crew and states what experience each
   kerbal is expected to gain, so that assignments serve C11 as well as the mission. The
   crew records of the agency record carry each kerbal's experience by body.

## Rehearsal and verification

The sandbox gives no rank, and the verification save may use the debug service's
`max_kerbal_experience` only to set up a test. Neither counts toward C11. Both exist to
rehearse and verify procedures, which the reports in `record/verification/` document.
