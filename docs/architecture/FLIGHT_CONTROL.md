# Flight Control

> **Navigation**: [Architecture](README.md) | [Harness design](HARNESS_DESIGN.md)

**Status.** Implemented development path, 2026-11-17, by Codex. The live path serves
one starting-parts verification article. It does not authorize career flight.

## Components

The 0BSD `fly` executable owns Keleusma compilation, execution, phase sequencing,
policy checks and the flight journal. The GPL-3.0-only `ikria-bridge` executable in
its separate repository owns the kRPC connection and game reference frames. They
exchange version 2 records over a loopback Transmission Control Protocol connection.
Version 1 and the original scripted host remain available for the timing experiment.

The bridge binds one vessel for one connection. It sends a hello, receives the
matching version, holds a physics tick, sends a sample, accepts exactly one matching
command, acknowledges completed calls and advances one tick. Sequence numbers begin
at zero. Game time must increase by between 1 and 30 milliseconds. Messages are
newline-terminated JSON, strictly below 64 KiB, with exact floating-point round trips.
A lost acknowledgement never permits a retry.

## Program contract

[dialogue.kl](../../programs/dialogue.kl) declares the version 1 flight-program
interface. The loader prepends it to a program body, checks the entry signature,
rejects native imports and persistent data, compiles with Keleusma 0.2.2 and verifies
with recomputed bounds. The workspace manifest requires exactly `=0.2.2`. It formerly
gave `0.2.2`, a caret requirement that admitted any later 0.2 release and was pinned
only by the lock file under `--locked`. Sources are limited to 64 KiB. The default admission limit
is 10,000 nominal cycles, with separate one MiB arenas for normal and abort programs.
This is a resource admission check, not a calibrated wall-clock guarantee.

The scheduler invokes each program only on its constructing thread. It requires one
yield and then one reset between inputs, decodes the output before resuming, and
permanently retires a faulted instance. The abort program is compiled before arming
and owns a separate arena. Changing to it does not reuse the faulted virtual machine.
Arbitrary in-place hot swaps and multiple vessels are not implemented.

[hop.kl](../../programs/hop.kl) decides ignition, coast, parachute deployment and
completion. Its phase input makes sequencing visible in the journal. Controls use
thousandths. Host conversion supplies altitude, vertical speed, fuel, situation and
vessel-frame direction and angular rates. The proportional-derivative attitude law
has saturation, but its actuation signs and gains remain unqualified in the game.
The empty starting capsule has no control source. Its continuous outputs are masked
to zero. Part-level ignition and deployment are verification instrumentation.

[abort.kl](../../programs/abort.kl) is the default abort program. It shares only the
dialogue with `hop.kl`. It never ignites and never commands a continuous control, so
it reads no attitude input and performs no attitude arithmetic. On the pad it completes
the pad abort. In flight it waits for burnout, deploys only inside the verified
envelope and reports completion only when the vessel is down. A test steps it over the
bounds of the validated telemetry domain and checks that it never faults and that the
shared policy permits every command it produces. Both programs still share the loader
and the Keleusma virtual machine, so a defect in either can fault both.

## Reference frames

Velocity is measured in the rotating body frame, whose origin is the body centre.
The vessel-moving surface frame is used only to identify local up. Up is transformed
into vessel coordinates, whose axes point right, forward and down. Angular velocity
is first measured relative to the rotating body frame and then transformed into
vessel coordinates. Reading either velocity in a frame moving with the vessel can
return zero. The first live run exposed this defect and lost its uncrewed article.
[VR-0010](../../record/verification/VR-0010.md) preserves that run and its correction.

The harness uses direction vectors and `atan2` rather than pitch and heading Euler
angles at vertical launch. Its pitch and yaw errors target local vertical. It damps
roll rate but does not hold an absolute roll orientation. These laws are development
software. The completed hop exercises discrete commands and passive fin stabilization.

## Authorization and aborts

The bridge identifies the loaded save by writing a uniquely named temporary snapshot,
locating it under the supplied saves directory and removing it. Only `Ikria Physical
Verification` is admitted. Arming additionally requires the named revision B on the
pad, eight expected parts, zero crew, 35 solid-fuel units, a 20 percent thrust limit,
an inactive motor and a stowed parachute. Built-in attitude assistance must be off.
The save-load counter, scene, active vessel, warp and physics state are checked each
tick. Part loss, crew aboard or changed identity terminates the session.

Both processes validate the complete command before applying it. The profile allows
one engine ignition on the pad or one safe parachute deployment after burnout,
descending below 1,500 metres and above 5 metres. Other discrete commands, translation
and wheel controls are refused. No career capability is inferred from the debug-rich
verification save. Ground program changes without a link are not exposed.

| Phase | Abort behaviour |
|---|---|
| Pad | Inhibit ignition and complete the pad abort |
| Boost | No further ignition, zero continuous commands, wait for burnout |
| Coast or descent | Deploy only inside the same verified deployment envelope |
| Touchdown | Require low vertical speed and one continuous second landed or splashed |

A solid motor cannot be stopped. A parachute cannot guarantee survival when the
vehicle is already too low or deployment is unsafe. A program fault selects the
precompiled abort program while telemetry remains valid. A program fault includes a
fault inside the normal program and any normal output that the session refuses, by
phase sequencing, the ignition rule, the completion guard or the authorization
policy. The refused command is never sent, and the abort program answers the same
sample, so no uncertain command is repeated. The session refuses any abort output that
ignites or commands a continuous control. A transport, telemetry or abort-program
fault ends the session. Neutral commands are not a universal safe state and cannot
replace a functioning descent program.

Ignition and deployment are one-shot. Once the session has decided to send a command
carrying either, it latches the action, before journalling the intent and before
sending, so that a failed journal or socket write still counts as an attempt. Programs and policy then see the action
as done even while its telemetry flag still lags, and a repeated request is refused.
The latch only adds refusals. The journal keeps the raw sample. The bridge still calls
the unlatched `authorize`, whose signature and behaviour are unchanged.

`IKRIA_ABORT_FILE` names an optional local file. Creating it requests an abort on the
next valid tick. A path that cannot be checked, for example because of a permission
error, also counts as a request. The request latches for the session and is
journalled. It is an operator interface for verification, not a career ground-command
interface.

## Failure and trust boundaries

The bridge allows 700 milliseconds for a held cycle against the server's observed
one-second hold timeout. The harness bounds wall time with a monotonic clock. Reading
one bridge frame may take at most two seconds and writing one harness frame at most
250 milliseconds, however the peer paces its bytes. The whole session may take at
most 720 seconds from connection, which is below the 840 seconds that
`tools/phase0/run_flight.py` allows the process. Each socket call's timeout is set to
the time left, and an expired deadline refuses every later call. A session that needs
longer therefore ends as a transport fault. Close is sent outside the session deadline,
with its own 250 millisecond write timeout set on the shared socket first. It is
withheld after a partial frame, which it would corrupt. A write refused at an expired
deadline before any byte left does not count as partial. Disconnection then
releases the tick. Both executables have bounded frame sizes and tick counts. Server timeout configuration
is an installation assumption, not a remotely negotiated guarantee. Commands are
sequential kRPC calls, not an atomic batch. A partial write can therefore leave an
uncertain result. The session stops rather than resending it.

A cancelled kRPC call can leave a partial response. That connection is discarded,
never reused for cleanup. On a healthy connection the bridge attempts neutralization
and release. Process exit disconnects the client, which also releases a held tick.
No stream update is awaited while the tick is held. The bridge's three asynchronous
framing tests require no game or network socket.

Loopback peers are trusted local processes, not authenticated users. Input validation
limits their capabilities but is not protection against another process owned by the
same account. This release exposes no debug setter, staging, launch, save loading,
career command, autopilot or arbitrary part event through the flight protocol.

The journal records configuration declared by the peer, program hashes, every sample,
command intent before sending, acknowledgement, faults and completion. Each frame read
from the bridge is journalled before it is validated, so a refused sample or an
unexpected acknowledgement remains evidence. A line that is JSON but not a bridge
frame is journalled as an unparsed frame, whole up to 4,096 bytes and otherwise as an
excerpt of that length with its full length, so the record stays below the frame bound.
A refused hello is journalled as such before the session ends. An accepted hello is
journalled inside the session header, which replay requires to be the first record.
Bytes that are not JSON, or that exceed the frame bound, cannot be journalled as a
record. Flush precedes
each command, with file synchronization at session end. Per-tick power-loss durability
is not claimed. Log write failure stops command production. The operating system and
filesystem can still stall, and no hard real-time guarantee is made.

## Running and replaying

Check out the two repositories as sibling directories named `ikria` and
`ikria-bridge`. Build each before starting the bridge. Prepare the empty article
using the guarded Phase 0 setup tools. Supply the actual local saves directory.

```sh
cd ikria-bridge
RUSTC_WRAPPER= cargo build --locked
./target/debug/ikria-bridge "$SAVES_DIRECTORY" IKR-TS-001B 50120 12005
```

That command is telemetry only. In a second terminal, from Ikria, run the monitor.
Use a new log path for every session. The bridge accepts only one connection. `tools/phase0/run_flight.py` can supervise
both processes with exclusive log files and bounded process lifetimes.

```sh
RUSTC_WRAPPER= cargo build -p ikria-harness --bins --locked
./target/debug/fly 127.0.0.1:50120 tmp/monitor.jsonl monitor 50
```

For an intentional verification flight, restart the bridge with
`--arm-verification` and run `fly` with `hop`. `pad-abort` tests inhibition without
ignition. The optional fifth `fly` argument is a program body with the same dialogue.

```sh
IKRIA_ABORT_FILE=tmp/request-abort ./target/debug/fly \
    127.0.0.1:50120 tmp/hop.jsonl hop 12000
./target/debug/replay tmp/hop.jsonl
```

Remove an old abort-request file before a new intended flight. Replay requires a
successful terminal record, a matching program hash and identical commands. It refuses
failed or truncated journals. Optional normal and abort program-body arguments permit historical replays, with
both hashes checked. Without an abort argument, replay selects the abort body whose
hash the journal records among the normal body, `abort.kl` and `hop.kl`. Journals
from before the distinct abort program therefore still replay, and `live-hop-3` and
`live-abort-4` were replayed identically after the changes of 2026-11-18. Replay reads at most 64 MiB and 100,000 records. It checks software decisions,
not the physical response of the game.

## Remaining qualification

The live verification record is not career clearance. Unresolved work includes a
campaign ruling for the starting capsule's absent control source, calibrated timing
admission, attitude-actuation tests, repeated qualification trials, additional abort
envelopes, science collection, a save-derived career authorization model, and the
mission controller. Multi-vessel scheduling, staging, general phase-module loading,
arbitrary in-flight program replacement and orbital guidance remain outside this
single-vessel implementation.

## Known defects

An independent review of the implementation on 2026-11-18 reported the defects below.
Items 1 and 2 were confirmed against the code by the receiving Claude line. The others
are the review's findings, confirmed by its reading or, for item 5, by mutation, and are
to be rechecked when fixed. They are to be fixed before further flights.

1. **An invalid program output ends the session without the abort program.** In
   `session.rs`, a refused phase transition, the completion guard and a refusal by
   `authorize` return an error, and only faults inside the normal program's step select
   the abort program. A lagging `chute_deployed` flag in phase 3, or a brief `Landed`
   report during boost, can then stop all commands in flight. This contradicts
   "A program fault selects the precompiled abort program". I-0002 was a live
   instance, in which the completion guard ended the session after a bounce. Its
   correction changed the program's logic and left this path unchanged.
2. **The abort program is the normal program's source.** The live default compiles
   `hop.kl` for both, in separate arenas with the `abort` input set. A sample that
   faults one will probably fault the other, which ends the session. The arena is
   independent, but the logic is not.
3. **Parachute deployment is not latched.** Neither the session nor `authorize`
   remembers a deployment already sent, so a lagging flag permits repeats.
4. **No deadline bounds a frame or a session in wall time.** Read timeouts bound each
   read call only.
5. **Some Python boundary tests cannot fail on the guards they name.** Comparisons
   between mock objects raise the expected error even with the guard removed, and
   attribute assignment on a mock is not recorded.
6. **`early_hop.py` records a landing at the first `Landed` sample,** without a settle
   period, so a later bounce or tip-over is recorded as intact.
7. **A refused sample is not journalled.** Validation runs before the sample is logged,
   although the journal is to record every sample.

Lesser items are that boost's zero continuous commands are enforced only by `hop.kl`,
that a Close frame can follow a partial command write, that `keleusma = "0.2.2"` is a
caret requirement pinned only by `--locked`, that the abort-request check treats a
permission error as no request, and that several tests pin constants derived from the
code's own formulas.

### Status after the fixes of 2026-11-18

The fixes are in commit `70ee0ef`, on a feature branch. Each fixed item has a test.
For items 1, 2, 3 and 7 and the abort-request check, the new session tests were also
run against the previous `session.rs` and failed there. They have not been exercised
in the game. No flight has used them, so the statement above that the defects are to
be fixed before further flights is met in code only, pending review.

| Item | Status | Change | Test |
|---|---|---|---|
| 1 | Fixed | `session.rs` treats a refused normal output as a program fault, journals it and lets the abort program answer the same sample. A refused abort output ends the session. | `refused_normal_output_selects_the_abort_program`, `refused_abort_output_ends_the_session_without_sending_it` in `crates/ikria-harness/tests/flight.rs` |
| 2 | Fixed | New `programs/abort.kl`, bundled as `flight::ABORT`, is the default abort program. `replay` selects the abort body by recorded hash. | `default_abort_program_is_not_the_normal_source`, `sample_that_faults_the_normal_program_does_not_fault_the_abort_program`, `abort_program_is_total_and_inside_policy_over_the_validated_bounds` |
| 3 | Fixed | `Commanded` and `authorize_latched` in `crates/ikria-records/src/flight.rs`, used by the session. `authorize` is unchanged for the bridge. | `lagging_parachute_flag_cannot_cause_a_second_deployment`, and two latch tests in `crates/ikria-records/tests/flight.rs` |
| 4 | Fixed in the harness | `crates/ikria-harness/src/transport.rs` adds frame and session deadlines, used by `fly`. | `crates/ikria-harness/tests/transport.rs`, with a simulated clock and stream |
| 5 | Fixed | `tools/phase0/tests/test_early_hop.py` uses fakes with real values and recorded actions, with a control run for each guarded action. | Mutation study below |
| 6 | Fixed | `early_hop.py` requires three seconds of game time down and slow, counts bounces, and records no outcome for a run that ends unsettled. | `TouchdownTests`, `HopLandingTests` |
| 7 | Fixed | The session journals each frame before validating it, and journals a JSON line that is not a bridge frame as unparsed. | `refused_samples_and_frames_are_journalled_before_refusal` |
| Boost commands | Fixed | The session refuses any continuous command while aborting. | Covered by the abort tests above |
| Close after a partial write | Fixed | `fly` withholds Close when the writer is inside a frame. | `partial_write_is_cut_and_reported_as_mid_frame` |
| Caret requirement | Fixed | `keleusma = "=0.2.2"` in `Cargo.toml`. `Cargo.lock` is unchanged. | The gate with `--locked` |
| Abort-request permission error | Fixed | `session::abort_file_requested` counts an unreadable path as a request. | `abort_request_file_fails_closed` |
| Tests that pin derived constants | Open | Not addressed. | None |

The mutation study for item 5 deleted each guard in a scratch copy of `tools/phase0`
outside the repository and ran the suite. The rewritten suite caught 41 of 42
deletions, covering 13 guards and 3 landing checks in `early_hop.py`, 9 guards in
`setup_early_hop.py` and 17 in `launch_site.py`. The survivor removes the check that
the temporary save appeared exactly once. It is equivalent, because an absent save
then raises an index error that the module also refuses. The previous suite, run the
same way against the previous scripts, caught 1 of the 21 deletions that applied to
them, the save guard of placement.

A second review of the harness changes, the same day, found no path that sends a
refused command or retries an uncertain one. Its five findings were fixed in a later
commit on the same branch. The refusal of continuous commands while aborting is now
tested. The abort program's sweep runs the session's own decision, `session::decide`,
over phase 0, the phase table and the completion guard, in every situation and with
and without control authority, and a direct test checks the guard and the table. A
refused hello and an unparsed line near the frame bound are journalled. A write
refused before any byte left no longer withholds Close. Each new test was checked to
fail with its fix removed.

The following remain open or unverified.

- An abort after an acknowledged ignition, with the vessel still reporting PreLaunch,
  for example because the motor did not light, does not end as a pad abort. The abort
  program holds phase 5 and the session ends at the tick budget with an error.

- The deadline bounds are tested only with a simulated stream and clock. The constants
  in `fly` have not been observed against the bridge or the game.
- The abort program shares the dialogue, loader and Keleusma virtual machine with the
  normal program, so a defect in any of them can still fault both.
- The bridge keeps calling the unlatched `authorize`. Adopting `authorize_latched`
  with its own latch would add defence in depth there. No protocol change is needed.
- Bytes that are not JSON, or that exceed the frame bound, are refused without a
  journal record of their content.
- Rule R20 is applied through `tools/phase0/launch_site.py`, which every launching
  script in `tools/phase0` and `tools/sepratron/flightlib.py` now uses. It considers
  only crewed vessels before launch or landed on Kerbin within 300 metres of the pad
  or inside the runway rectangle. In the flight scene it recovers them. In any other
  scene, where kRPC 0.7.0 refuses to list vessels, it reads a temporary save, which it
  always deletes, and refuses the launch if one is found. The launch areas come from
  the game's waypoints and measured positions, and the save-file reading is tested only
  against a small fixture, not a save written by the game. The phase 0 sandbox probes
  take no saves directory, so outside the flight scene they refuse to launch.
