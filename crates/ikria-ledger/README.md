# ikria-ledger

The science ledger, roadmap task T9. It enumerates every science subject that the
Phase 0 count of [`VR-0005`](../../record/verification/VR-0005.md) counts, one by one,
with the identifier the game writes in a save and its value. It then joins that
enumeration with a save's science subjects, read through
[`ikria-save`](../ikria-save/README.md), and gives each subject's possible, collected and
remaining science, with totals by body, experiment, situation and rule. It serves
campaign criterion C4, which asks that every subject reach its cap. It never writes.

## Use

```rust
use ikria_ledger::{Enumeration, Ledger, ScienceScale};
use ikria_save::Save;

let enumeration = Enumeration::phase0()?;
let save = Save::read(std::path::Path::new("persistent.sfs"))?;
let ledger = Ledger::from_save(&enumeration, &save)?;
let (possible, collected, remaining) = ledger.totals().points(ScienceScale::HARD);
```

```sh
RUSTC_WRAPPER= cargo run -p ikria-ledger --example ledger_summary -- path/to/save.sfs
```

## Why the rules are ported rather than generated

The task offered two designs. One was to port the rules of
[`science_subjects.py`](../../tools/phase0/science_subjects.py) to Rust with tests that
hold the two in agreement. The other was to have the script emit the subject list as a
tracked data file for the crate to read. The port was chosen for three reasons.

1. **The script is a closed record.** VR-0005 names it in its final version, and it
   reads an untracked `tmp/inventory.json`. Making it emit a list would change a record
   after the fact.
2. **The script lacks what the list needs.** It holds the location biomes and the surface
   features as counts only, and its split of the features by cap is wrong, as described
   below. A generated list would have needed new rules in the script either way.
3. **Types and review.** The port gives bodies, situations and experiments as enumerations,
   holds every value in exact hundredths, and needs no parsing of data at run time. The
   tables are short enough to review in [`src/catalog.rs`](src/catalog.rs).

Agreement is held by [`tests/phase0.rs`](tests/phase0.rs). It parses the script's
`EXPS`, `MULT`, `HARD`, `KSC_BIOMES`, mask bits and `ROC_CAPS_TOTAL` from the tracked
script text and compares them with the port, compares every body's biomes and atmosphere
with the tracked `inventory.json`, and recomputes the record's totals from the
enumeration.

## The acceptance and the surface feature erratum

The acceptance of T9 is that the totals match the Phase 0 count. The tests show the
following.

| Figure | Phase 0 record | Enumeration | Agreement |
|---|---|---|---|
| Subjects in total | 3,104 | 3,104 | Exact |
| Body subjects, and each body's count | 2,700 | 2,700 | Exact |
| Body value on Hard, and each body's value | 215,679 | 215,679 | Exact, each body to the tenth |
| Location subjects | 370 | 370 | Exact |
| Location value on Hard | 1,239 | 1,239 | Exact |
| Surface feature subjects | 34 | 34 | Exact |
| Surface feature value on Hard | 900 | 909 | Differs by 9 |
| Total value on Hard | 217,818 | 217,827 | Differs by 9 |

**Fact.** Breaking Ground's `ScienceDefs.cfg`, read on 2026-11-18, defines 34 surface
features with caps of 30 for twelve, 45 for eleven and 60 for eleven, which is 1,515.
The Phase 0 script's `ROC_CAPS_TOTAL` was `30 * 12 + 45 * 12 + 60 * 10`, which is 1,500,
where the game defines 12 features at 30 and 11 each at 45 and 60, which is 1,515. The
enumeration found the difference of 9 on Hard. VR-0005, the script and its data were
corrected on 2026-11-18, and the tests now require exact agreement.

## Identifiers

A subject's identifier is `experiment@BodySituationBiome`, where the situation is one of
`SrfLanded`, `SrfSplashed`, `FlyingLow`, `FlyingHigh`, `InSpaceLow` and `InSpaceHigh`.
The biome is omitted when the experiment does not depend on the biome in that
situation, or the body has none. Otherwise it is the biome's name with spaces removed.

- **Fact.** The operator's older career save held 177 subjects, all at Kerbin. Of
  these, 174 matched the enumeration exactly and none had a cap that differed from it.
  The other 3 were recovery subjects, which the count leaves out. The matched forms
  include biome subjects in the landed, splashed, flying low and space low situations,
  subjects without a biome, and the name `R&D` with its ampersand.
- **Inference.** The same form holds at the other bodies, with biome names from the
  inventory. No save read held a subject away from Kerbin.
- **Inference.** A surface feature's subject is `ROCScience_Name@BodySrfLanded`, since its
  definition has a situation mask of 1 and a biome mask of 0. Its body is the one its
  name gives, and `ROCScience_DunaEjectaOnIke` is taken to stand on Ike. No save read
  held one.

## Location biomes

Phase 0 counted 37 Space Center location biomes from the wiki, by number only. The
names come from three places, and each subject carries its name's provenance.

- **Observed, 12.** `KSC`, `Administration`, `AstronautComplex`, `Crawlerway`,
  `FlagPole`, `LaunchPad`, `MissionControl`, `R&D`, `Runway`, `SPH`, `TrackingStation`
  and `VAB` appear in the older career save's identifiers.
- **From the localization dictionary, 21.** The game's
  `GameData/Squad/Localization/dictionary.cfg` holds the display names of the observed
  locations, and of the parts of the building groups, in the entries `#autoLOC_6002105`
  to `#autoLOC_6002147`. The eight parts of `R&D`, four of `SPH`, four of
  `Tracking Station` and five of `VAB` are taken from there. That the identifier holds the
  display name with spaces removed is an inference from the twelve observed names, all
  of which follow that rule.
- **Missing, 4.** The same block holds eight further candidates, `Pad`,
  `Pad Flag Pole`, `Pad Water Tower`, `Pad Round Tank`, `Pad Tanks`,
  `Launchpad Platform`, `Baikerbanur` and `Baikerbanur LaunchPad`. Nothing read offline
  shows which four of them, if any, the wiki's 37 include, and the wiki refused an
  automated read on 2026-11-18. The four are placeholders named `<unnamed location 1>`
  to `<unnamed location 4>`. They keep the count and value correct, and they never match
  a save, even one that holds their identifier.

The biome names live in compiled game assets rather than configuration files, so the
files cannot settle the question. A save in which a kerbal has reported from each part
of the launch pad would.

## Units and the ledger's rules

- **Inference.** A save's `cap` and `sci` are in base units, before the difficulty's
  science multiplier. The same recovery subjects hold the same `cap` in saves at
  multipliers of 1 and 0.6, and the older save's 174 matched caps all equal the cap times
  the body multiplier with nothing else applied. The ledger works in base units, and
  `Totals::points` converts to points at a multiplier. `science_scale` reads the
  multiplier from `PARAMETERS`, `CAREER`, `ScienceGainMultiplier`.
- **Possible science** is the enumeration's value. Where the save's cap differs from it
  by more than a relative 1e-5, with a floor of 1e-4, the entry reports both values as
  `CapCheck::Differs`, and the ledger counts the differences. Neither value is chosen
  silently.
- **Remaining science** is the possible less the collected, never below zero, summed per
  subject. Science beyond the enumeration's value is reported as `Entry::excess`.
- **Unmatched and repeated subjects.** A save subject that matches nothing, such as a
  recovery or an asteroid sample, is listed with its science. A save subject that repeats
  an earlier identifier is listed as repeated, and the first is the one joined. Every
  save subject is accounted for once.
- **Matching is exact**, including case.

## Limits and failure modes

- **Scope.** Asteroid and comet samples, recoveries, deployed experiments and the
  laboratory are not enumerated, as in VR-0005. Their subjects appear as unmatched.
- **Rules from Phase 0.** The rules carry VR-0005's assumptions. Splashed subjects exist
  only at Kerbin, Eve and Laythe, and at every biome there, including biomes without
  water. Location biomes count for landed experiments only. The multipliers are the
  wiki's, not read from the game. A save that disagrees shows as cap differences or
  unmatched subjects, which is the intended way to find such errors.
- **Stock bodies only.** A modification that adds or renames bodies or biomes is not
  covered.
- **Panics.** No input is known to make the ledger panic. Building the enumeration can
  fail only on a defect of the tables, which a test excludes, and it returns an error
  rather than panicking.

## Tests

`tests/phase0.rs` holds the acceptance totals, the per-body figures, the agreement with
the script and the inventory, the identifier form on observed and constructed examples,
and identifiers the rules exclude. `tests/ledger.rs` joins a hand-written fixture,
`tests/fixtures/ledger.sfs`, that holds a partly collected subject, a collected one, two
cap differences, one with collection beyond the enumeration's value, an asteroid sample,
a recovery, a repeated identifier, an identifier differing only in case and a
placeholder identifier. It also covers a save without subjects, a missing or invalid
science multiplier, a save without research, and a malformed subject.

**Coverage gaps.** No test runs against a save written by the game, since none is
committed. The run on the older career save is recorded above and covers Kerbin only.
No identifier away from Kerbin, of a surface feature, or of a location other than the
twelve observed has been checked against a real save.
