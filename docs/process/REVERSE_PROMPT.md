# Reverse Prompt

> **Navigation**: [Process](README.md)

Written by Codex on 2026-11-17. This state file replaces the earlier rehearsal report.

## Requested work and result

The operator requested the software needed for Keleusma flight-control loops. This
branch builds the live single-vessel verification path on local T10 and rehearsal
work. The paired separate bridge branch is `feat/codex-live-bridge`. Neither repository
was pushed. Main was refreshed and remains `1ce12bb` here and `551bb7b` in the bridge.

The implementation adds the bounded version 2 flight protocol, a guarded Rust kRPC
bridge, checked Keleusma program loading and nominal resource admission, hop phases,
an independent precompiled abort program, a latched external abort request, strict
command policy, timeouts, write-ahead intent journals and deterministic replay.
The bridge uses explicit rotating-body velocity and transforms rates and local up
into vessel coordinates. Career writes are disabled.

## Observed verification

A normal flight and a coast-abort flight completed with all eight parts and were
recovered. Their 6,824 and 6,798 samples replay to identical commands. A telemetry-only
session and pad abort also completed and replayed. VR-0010 retains compressed journals,
program hashes, command events, setup, recovery and outcome metadata. M-0001 supplies
the provisional mission report and the vehicle sheet is updated.

The development series also exposed two substantive faults. I-0001 records an empty
article lost because moving-frame velocity prevented descent detection. The recovery
probe repeated the same frame mistake. I-0002 records an intact landing whose bounce
stopped the harness. Both were corrected, regression-tested and followed by completed
flights. Earlier monitoring also exposed JSON time round-trip precision, now tested
over 10,000 timestamps. Failed evidence was retained and is not certified by replay.

The staged local gates cover 60 Rust and 13 Python tests in Ikria and three bridge
tests, with no ignored cases, plus format, lint and repository checks. The local
platform was macOS with Rust 1.100.0. No remote continuous integration was observed.

## Final state and limits

The game is at the verification Space Center. Jebediah is available. The career and
craft hashes match their initial values. Five debug flags read false before and after
the series, and no debug setter was used. No career flight occurred.

The live capsule had no control source or link. Its continuous controls were zero.
The record establishes discrete ignition, deployment and phase decisions under
Keleusma, not qualified attitude actuation. One live abort was in coast, not boost.
Broader abort envelopes, statistical repeatability, career authorization, calibrated
timing, the original sandbox profile, multiple vessels and the mission controller
remain open. The operating guide names these limits explicitly.

## Preserved open work and next action

T8, T9, the remaining T12 qualification, the paused Green Monolith procedure, E1 and
E2 vehicle designs, program and financial article checks, the template upstream
decision, and broader assistant push authority remain open. T11 and the new scoped
T17 are complete locally. Prior T10 and T16 remain in this branch's history.

Ask before pushing. If approved, publish the Ikria branch before the bridge branch,
whose continuous integration pins the corresponding records revision. Review both
repositories and the two earlier local commits included by this branch.
