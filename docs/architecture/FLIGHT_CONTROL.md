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
with recomputed bounds. Sources are limited to 64 KiB. The default admission limit
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
precompiled abort program while telemetry remains valid. A transport, telemetry or
abort-program fault ends the session. Neutral commands are not a universal safe
state and cannot replace a functioning descent program.

`IKRIA_ABORT_FILE` names an optional local file. Creating it requests an abort on the
next valid tick. The request latches for the session and is journalled. It is an
operator interface for verification, not a career ground-command interface.

## Failure and trust boundaries

The bridge allows 700 milliseconds for a held cycle against the server's observed
one-second hold timeout. The harness imposes socket read and write timeouts. Both
executables have bounded frame sizes and tick counts. Server timeout configuration
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
command intent before sending, acknowledgement, faults and completion. Flush precedes
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
both hashes checked. Replay reads at most 64 MiB and 100,000 records. It checks software decisions,
not the physical response of the game.

## Remaining qualification

The live verification record is not career clearance. Unresolved work includes a
campaign ruling for the starting capsule's absent control source, calibrated timing
admission, attitude-actuation tests, repeated qualification trials, additional abort
envelopes, science collection, a save-derived career authorization model, and the
mission controller. Multi-vessel scheduling, staging, general phase-module loading,
arbitrary in-flight program replacement and orbital guidance remain outside this
single-vessel implementation.
