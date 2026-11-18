# Ikria

[![CI](https://github.com/sgeos/ikria/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sgeos/ikria/actions/workflows/ci.yml)
[![License: 0BSD](https://img.shields.io/badge/license-0BSD-blue.svg)](LICENSE)

<!--
Add a badge for each place the project is published, after the first release, for example:
[![Crates.io](https://img.shields.io/crates/v/ikria.svg)](https://crates.io/crates/ikria)
[![Docs.rs](https://docs.rs/ikria/badge.svg)](https://docs.rs/ikria)
[![PyPI](https://img.shields.io/pypi/v/ikria.svg)](https://pypi.org/project/ikria/)
[![npm](https://img.shields.io/npm/v/ikria.svg)](https://www.npmjs.com/package/ikria)
-->

A Keleusma technology demonstrator that flies Kerbal Space Program craft through kRPC, with an MCP server for agents.

## Status

**Early development.** The scripted harness and a single-vessel flight path are
implemented, and verification flights have been flown in a test save. No career mission
is cleared yet. What Ikria is, and the rules it follows, are in its charter,
[`docs/decisions/PROJECT_CHARTER.md`](docs/decisions/PROJECT_CHARTER.md). The first
milestone is in [`docs/roadmap/`](docs/roadmap/README.md).

The name is the Greek word for the land-based rowing benches on which crews learned to
row in time.

## Development

```sh
./scripts/check.sh                       # the local gate, which continuous integration also runs
scripts/worktree.sh new feat/some-thing  # an isolated worktree for a parallel line
scripts/scratch.sh status                # whether build output is linked to a scratch volume
```

## Documentation

The documentation index is [`docs/README.md`](docs/README.md). Agent guidance is in
[`CLAUDE.md`](CLAUDE.md), and [`AGENTS.md`](AGENTS.md) points other assistants to it.
Participation is governed by the [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

[0BSD](LICENSE).
