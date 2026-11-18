# ikria-save

A read-only reader for Kerbal Space Program 1 save files, roadmap task T8. It parses
the game's ConfigNode text format and extracts the career state as typed records, which
the science ledger of task T9 builds on. It has no dependencies beyond the standard
library, and it never writes.

## Use

```rust
use ikria_save::Save;

let save = Save::read(std::path::Path::new("persistent.sfs"))?;
let agency = save.agency()?;
let ledger_input = save.science_subjects()?;
```

```sh
RUSTC_WRAPPER= cargo run -p ikria-save --example save_summary -- path/to/save.sfs
```

| Extractor | Source in the save | Record |
|---|---|---|
| `agency` | `Funding` `funds`, `ResearchAndDevelopment` `sci`, `Reputation` `rep` | `Agency` |
| `facilities` | `ScenarioUpgradeableFacilities`, one child per facility with `lvl` | `FacilityRecord` |
| `technologies` | `ResearchAndDevelopment`, `Tech` nodes with `id`, `state`, `cost`, `part` | `Technology` |
| `science_subjects` | `ResearchAndDevelopment`, `Science` nodes with `id`, `title`, `sci`, `cap`, `dsc`, `sbv`, `scv`, `asc` | `ScienceSubject` |
| `roster` | `ROSTER`, `KERBAL` nodes | `Kerbal` |
| `vessels` | `FLIGHTSTATE`, `VESSEL` nodes, with the body from `ORBIT` `REF` | `Vessel` |
| `contracts` | `ContractSystem`, `CONTRACT` nodes in `CONTRACTS` and `CONTRACTS_FINISHED` | `Contract` |

A kerbal outside a vessel and a planted flag are vessels of the types `EVA` and `Flag`,
and `Vessel::is_eva` and `Vessel::is_flag` pick them out.

## Format assumptions

The structure was learnt from a save written by game version 1.12.5. Facts are what that
save showed. Inferences are marked, and each is also marked where the code relies on it.

- **Fact.** A save has one top-level node, `GAME`. Values are `key = value` lines, and a
  node is a name on one line followed by `{` on the next and closed by `}`. Keys and
  child nodes repeat, as `part` does within a `Tech` node, and a part may be listed
  twice in one node.
- **Fact.** That save had no `//` comment and no brace sharing a line with other text.
  The reader still accepts comments, and the form `NAME {` on one line.
- **Inference.** `//` begins a comment wherever it appears, as in the game's own reader,
  so a value containing `//` is cut short. A value runs to the end of its line, so it may
  contain `=` signs, spaces and braces.
- **Inference.** A facility's `lvl` is its level index divided by the highest index.
  Every stock facility is taken to have three levels, so 0, 0.5 and 1 are the levels
  displayed as 1, 2 and 3. A fraction more than `FacilityLevel::TOLERANCE` from those is
  refused, not rounded, because it would mean the assumption is wrong. Whether the flag
  pole has three levels has not been verified.
- **Inference.** The twelve numbers of a contract's `values` are, in order, the offer's
  duration, the contract's duration, the funds on acceptance, completion and failure,
  the reputation on completion and failure, the science on completion, and the dates of
  expiry, acceptance, deadline and finish. The magnitudes in the save studied fit this
  order. The rewards of a contract's `PARAM` nodes are not read.
- **Inference.** The finished contracts are kept in a node named `CONTRACTS_FINISHED`,
  which the save studied did not contain.
- **Inference.** Body indices follow the stock order from the Sun at 0 and Kerbin at 1 to
  Eeloo at 16. The reversed order of the bodies in the save's contract weights, the
  contracts naming Kerbin as body 1, and the Minmus ascents of the Sepratron program at
  body 3 all fit it. A modification that adds bodies breaks `BodyIndex::stock_name`.
- **Inference.** Game version 1.12.5 writes no experience field on a kerbal, and derives
  experience from the career log. `experience` and `experienceLevel` are read where
  present, and the career log's flight count is read.
- **Fact.** The save studied held no kerbal outside a vessel and no flag. Their fixture
  follows the format's known structure, with the kerbal's name in the vessel's `name`
  and in its part's `crew`.

## Limits and failure modes

- **Size.** Input over 64 MiB by default is refused before it is decoded. A file is read
  at most one byte past the limit, so a file that grows while it is read stays bounded.
- **Depth.** Nesting deeper than 32 by default is refused, counting `GAME` as depth 1.
  The save studied nested 9 deep. The limit may be raised to 256 and no further, because
  dropping a parsed tree recurses once per level.
- **No recursion while parsing or writing.** Both use an explicit stack.
- **Typed errors.** A parse error names its line. An extraction error names its node,
  field and line, and quotes at most 64 characters of the offending value. The first
  error stops an extractor.
- **Unknown fields and nodes are ignored.** So are facilities other than the stock ten,
  and kerbal traits other than the stock four are kept by name.
- **Closed sets are enforced.** A vessel type, situation, roster status, kerbal type,
  technology state or contract state outside the stock values is an error, since the
  game's own sets are closed. A modification that extends one would need this crate
  changed.
- **Repeats.** Where a node or key expected once is repeated, the first is used.
- **Text.** Input must be UTF-8. A leading byte order mark is skipped, and line endings
  may be a line feed or a carriage return and line feed.
- **Writing back.** `Node::to_config_text` writes values before child nodes, so the
  relative order of values and nodes is not kept. It exists to test the parser and is
  not meant for writing saves.

## Tests

The tests run offline against hand-written fixtures in `tests/fixtures`, which follow the
structure of a real save but hold invented values and only the stock kerbals. They cover
each extractor on its fixture, a write and reparse of every fixture, and the refusals of
malformed lines, unbalanced braces, the size and depth limits, invalid UTF-8, missing
nodes and fields, and values of the wrong form.

**Coverage gaps.** No test runs against a save written by the game, since no save is
committed. A save from another game version or with modifications may differ in ways
the fixtures do not show. The claim that no input makes the reader panic rests on review
and on these tests, not on fuzzing.
