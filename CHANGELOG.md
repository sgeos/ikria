# Changelog

All notable changes are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- The science ledger, `ikria-ledger`, which enumerates the 3,104 subjects of the Phase 0
  count with their identifiers and Hard values, and joins them with a save's subjects
  read through `ikria-save`. It finds that the count's surface features are short by 9
  science on Hard.

- A read-only save-file reader, `ikria-save`, with a bounded iterative ConfigNode
  parser and typed extractors for the agency's totals, facility levels, technologies,
  science subjects, the crew roster, vessels including kerbals outside a vessel and
  flags, and contracts, tested offline against hand-written fixtures.

- A guarded live flight protocol, verified Keleusma hop and abort programs, bounded
  transport, command journalling, deterministic replay and live verification evidence.

- Two starting-parts hop development runs in the verification save, with guarded
  probes, offline tests, telemetry and a provisional vehicle specification.

- A single vessel harness skeleton, a scripted host, a Keleusma rate-damping adapter,
  deterministic replay and failure tests, and a runnable `scripted_host` example.
- The repository, seeded from `sgeos/project-template` at `3954eba3268770cff812cd7cd79996221637a215`, tag `v1.0.8`.

