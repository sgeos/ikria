# Harness Design

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

**Status.** Proposed, 2026-11-14, for the operator's review, with the process layout and
the bridge's repository decided by the operator the same day. Nothing is implemented.
Each point is marked **Decided**, **Proposed** or **Open**. Facts about Keleusma come
from a read-only reading of its repository at version 0.2.2 and the unreleased 0.2.3
work, with file references kept in this line's notes. Facts about kRPC come from the
installed 0.7.0 server and client and from its documentation.

This document designs the software that lets Keleusma control loops fly the campaign of
[`CAMPAIGN_PLAN.md`](CAMPAIGN_PLAN.md) and feed the record of
[`AGENCY_RECORD.md`](AGENCY_RECORD.md). It covers the three parts of the operator's
brief, the bridge to the game, the harness that runs the loops, and the mission
controller with its MCP tools. It also covers the record types between them, the
stepping contract, and how the rules of play are enforced.

## Overview

```
 agents ─MCP─▶ mission controller ─IPC─▶ harness ─IPC─▶ bridge ─kRPC─▶ KSP
                     │                      │
                     ▼                      ▼
                record/ ledgers       telemetry logs
```

| Part | Runs as | Licence | Knows about KSP | Owns |
|---|---|---|---|---|
| Bridge | Its own process, from its own repository | GPL-3.0 | Yes, the only part that does | kRPC connections, tick hold, translation to and from Ikria's record types |
| Harness | One process | 0BSD | No | Keleusma VMs, the tick scheduler, rule enforcement, telemetry logs, save snapshots |
| Mission controller | A second process | 0BSD | No | Agency state, mission sequencing, program-level loops, the MCP server, the record generators |

**Decided by the operator, 2026-11-14.** Three processes. The bridge is separate because the kRPC client and
service definitions it needs are GPL-3.0, as the bridge section explains, and charter
item 6 keeps GPL code apart. A process boundary keeps the harness and the controller
0BSD as built programs, not only as source. The harness and the controller are separate
so that a controller can be restarted, or an agent session can end, without disturbing
a vessel in flight. This answers open question 2 of [`OPEN_QUESTIONS.md`](OPEN_QUESTIONS.md).
The harness owns the shared clock, which is the game's universal time as the bridge
reports it. The messages between the processes are Ikria's own record types, defined
in a 0BSD crate.

## The host abstraction

**Decided by the handoff, carried here.** Everything above the bridge speaks Ikria's
own record types and never a kRPC type. A Rust trait, here called `Host`, stands between
the harness and any source of telemetry. It has two implementations.

- **The KSP host,** which is the bridge.
- **A scripted host,** which replays recorded telemetry or runs a simple model. It lets
  the harness, the rule enforcement and the MCP tools be tested without the game
  running, and makes those tests deterministic.

## Keleusma in the harness

### How a control loop runs

*Facts, Keleusma 0.2.2.* A control loop is a Keleusma `loop` stream processor. The
host compiles it to a module, verifies it, and creates a VM over an arena the host owns.
Each step, the host calls `resume` with the input, and the VM returns
`Yielded(output)`. At the end of the loop body the VM returns `Reset`, and the next
`resume` starts the body again. The verifier rejects any program whose worst-case
execution time or memory it cannot bound, and `wcet_per_iteration` reports the bound in
nominal cycles.

**Proposed.** Each physics tick, the harness resumes each loop once with a telemetry
record and receives one command record. A composite value yielded by the VM lives in
its arena until the next resume, so the harness decodes each command before resuming
again, as Keleusma requires.

### Programs come in pairs

The charter's goal is vehicle-control routine pairs with the sequencing that makes them
work. In the harness, a routine is a Keleusma program bound to one flight phase of one
vehicle, such as ascent, circularisation, descent or ascent from a surface. The
sequencing is a phase table. It names each phase, its program, its entry and exit
conditions, and its abort target. The harness hot swaps programs at the phase
boundaries.

### Hot swap

*Facts.* Keleusma swaps a module only at `Reset`, and expects the input and output
types to stay the same across the swap. The implementation does not check either
condition. It leaves both to the host.

**Proposed.** The harness checks both. It swaps only when the VM has just returned
`Reset`. All programs share one versioned dialogue, the telemetry record in and the
command record out, defined once in a Keleusma source file that every program
includes. The harness compares the dialogue version each module declares with its own
before it swaps, and refuses a mismatch. After a swap it calls the new entry point, as
Keleusma requires.

### Timing and admission

*Facts.* At 50 physics ticks per second, a tick lasts 20 ms of game time. Keleusma
reports bounds in nominal cycles, which a measured cost model from `keleusma-bench`
converts to time.

**Proposed.** The harness admits a program only if its calibrated bound, added to the
bounds of the programs already running, fits a share of the tick that leaves room for
the bridge. The share is to be set by measurement in Phase 0. A program the harness
cannot admit is not flown.

### Threads

*Fact.* A Keleusma VM and its arena are neither `Send` nor `Sync`, so each must live on
one thread for its whole life.

**Proposed.** One scheduler thread owns every VM and steps them in a fixed order each
tick, which keeps a tick's results independent of thread timing. The exchange with
the bridge process runs on another thread, connected to the scheduler by channels.
Several vessels at once therefore cost scheduler time, not threads.

### Faults

*Facts.* Keleusma reports a runtime fault as a `VmError` with a category, after which
the VM is in an undefined state until reset.

**Proposed.** A fault in a control loop puts its vessel into a safe state defined in
the vehicle's specification sheet, for example zero throttle with the controls
centred. The harness then starts the phase's abort target if one is defined, logs the
fault, and raises an incident under R11 if the fault affects crew or a save. A loop
that faults is not resumed until a person or the controller decides.

### Program-level loops

The operator's brief asks for space-program-level driver loops. *Fact.* Keleusma's
`impure loop main`, intended for such drivers, is planned for its 0.5.0 release and is
not implemented. **Proposed.** Until then, the controller's program-level loops are
ordinary `loop` stream processors, stepped on agency events rather than physics ticks.
Their input is agency state, such as funds, science, contracts and the fleet, and their
output is decisions for the controller to carry out. Their effects on the world pass
through the controller, which keeps them inside the rules.

## Record types

**Proposed, version 1.** These are sketches. The final fields follow the first
qualification flights.

**Telemetry in, per vessel, per tick.** Game time. Vessel identifier and situation.
Body and position and velocity in that body's inertial frame. Attitude as a quaternion
and angular velocity. Mass, available thrust and specific impulse of the active stage.
Altitude above the surface and above sea level, vertical and surface speed, dynamic
pressure. Resource totals, the current stage, and whether the vessel has a link. Each
field is filtered under R9 before the loop sees it.

**Commands out, per vessel, per tick.** Throttle. Pitch, yaw and roll. Translation on
three axes. Wheel throttle and steering. Requests to stage, to fire an action group,
to run or store an experiment, and to abort. Each request passes the R2 check before
the bridge sends it.

**Agency state in, per event, for the controller's loops.** Funds, science,
reputation, facility levels, active contracts, the fleet and the crew, all read from the
save file or kRPC.

## Enforcing the rules of play

| Rule | Mechanism in the harness |
|---|---|
| R2, capability parity | A capability table, rebuilt from the career state at each scene change, lists what the career has unlocked. Every command and every kRPC call passes it. Unlisted calls are refused. Guidance and attitude control are computed in Keleusma, and kRPC's built-in autopilot is not used. |
| R3, signal | Before forwarding a command to an uncrewed vessel, the harness checks the bridge's link status, and refuses the command without a link. |
| R9, information parity | Telemetry passes a filter before any loop sees it. A loop on board sees its own vessel's full state. The controller sees an uncrewed vessel only while it has a link. Bodies' motion is known from the start. Terrain, biomes, anomalies, asteroids and comets are revealed only by the events that reveal them in the game. |
| R10 and R11, losses | The harness quicksaves into the sandbox and snapshots the career save outside the game before each career flight. It restores a career snapshot only after the operator confirms a defect, and every restore and loss writes an incident report. |
| No cheats | kRPC's `KRPC.Debug` service sets funds and science, upgrades facilities, unlocks the tree, and moves vessels into orbits or onto the ground at will. The harness never calls it against the campaign's career or its sandbox qualification runs, and refuses any request that would. It may be used only in the test career save, for setting up a test, and each use is logged. |
| R7, configuration record | Each flight logs the game version, the settings, and the hash of every craft file and Keleusma module flown. |

## The bridge to kRPC

*Facts, from kRPC's documentation and its release of 2026-10-21, read 2026-11-14, and
from the installed server.*
- **Protocol.** A client opens a call connection on port 50000 and an optional stream
  connection on 50001, both on `127.0.0.1` here. Messages are Protocol Buffers with a
  varint length prefix. One request may carry several procedure calls, answered in
  order. Version 0.7.0 also offers a Unix domain socket, which its authors report
  carries about 20 percent more calls per tick. The installed server starts only once a
  game is loaded, not at the main menu.
- **When calls run.** The server runs calls in the game's fixed update, before the game
  collects control inputs, so a control sent in a tick takes effect in that tick.
  Streams update at most once per fixed update.
- **Tick hold.** Version 0.7.0 adds `HoldTick`, `ReleaseTick` and `NextTick`, which hold
  the game on one physics tick so that a read, compute and write cycle completes within
  it. Streams do not update while a tick is held, so a held tick is read by calls. The
  hold times out after one second by default, the installed `tickHoldTimeout`. kRPC's
  authors call it a stopgap that may change in later releases.
- **Rate.** *Measured 2026-11-14 in the sandbox save.* The game's fixed update is 0.02 s
  of game time, which is 50 per second. kRPC's documentation says about 60, which this
  installation does not bear out.
- **Rust clients.** `krpc-client` 0.7.0 of 2026-11-01 targets kRPC 0.7.0, supports
  streams, and generates its bindings from kRPC's service definitions. It is GPL-3.0.
  `stayputnik` is MIT or Apache-2.0 but predates kRPC 0.7.0's protocol changes. The
  others are GPL-3.0 and dormant.
- **Licences in kRPC 0.7.0.** The release is LGPL-3.0, except that the space centre and
  debug services, including the service definition for vessel control, are GPL-3.0,
  and the protocol schema is MIT. Later changes on kRPC's main branch, not yet
  released, relicense some clients and add a permission for generated client code.

**Decided by the operator, 2026-11-14.** The bridge is its own GPL-3.0 process in a
small separate repository, and uses `krpc-client`. The repository is
[`sgeos/ikria-bridge`](https://github.com/sgeos/ikria-bridge), public, seeded the same
day by this line from `sgeos/project-template` v1.0.8 and licensed GPL-3.0-only. That is the simplest licensing position and follows charter item
6. The alternative is a 0BSD bridge written from the MIT protocol schema, calling
procedures by name and generating no bindings from the GPL definitions. It would let
the bridge live in this repository, but whether it fully avoids the GPL is a licensing
judgement this line cannot make. It could replace the first bridge later behind the
same interface.

## Stepping contract

**Proposed.** Lockstep is the default, using kRPC's tick hold.

1. The bridge holds the tick and reads every vessel's telemetry in one batched request.
2. It sends the records to the harness.
3. The harness filters them under R9, steps every loop once, checks each command under
   R2 and R3, and returns the commands.
4. The bridge applies the commands in one batched request, then advances one tick.

In lockstep, a slow cycle slows the game rather than letting it run ahead, so timing
becomes a matter of throughput rather than correctness. The one-second hold timeout
bounds a stalled harness. If the hold proves unusable, the harness runs free, stepping
on each streamed sample and logging the delay from sample to command. Every record
carries game time in both modes, because the game slows under load rather than
lengthening its ticks.

## The MCP tools

**Proposed.** The tools speak the agency's terms, never kRPC's, so an agent works the
same way whatever host is behind the harness.

| Tool | Purpose |
|---|---|
| `agency_status` | Funds, science, reputation, facilities, crew and active contracts |
| `ledger` | The standing of the completion criteria and the science ledger |
| `fleet` | Vehicles in flight with their phase, program and health |
| `vehicle_state` | One vehicle's filtered state, under R9 |
| `mission_plan` and `mission_report` | Read and write the mission documents of the record |
| `qualify` | Run a vehicle's profile in the sandbox under R4 and return the result |
| `launch` | Start a qualified mission in the career, after the checks of R4 and R12 |
| `swap_program` | Request a program change at the next safe point |
| `abort` | Command a vehicle's abort target |
| `incident` | Open an incident report under R11 |

Tools that change the career, such as `launch`, `abort` and any restore, are gated as
the rules require, and every call is logged to the record.

## Repositories and crates

**Proposed.** This repository holds the 0BSD crates, which are `ikria-records` for the
record types and the messages between processes, `ikria-harness`, `ikria-controller`
with the MCP server, and `ikria-record` for the generators. The bridge lives in a
separate GPL-3.0 repository, here called `ikria-bridge`, which depends on
`ikria-records` and `krpc-client`. Keleusma is used at its released version 0.2.2,
which is 0BSD, and the harness follows its later releases deliberately.

## Phase 0 experiments

Each answers a question this design depends on. All run in the sandbox save. The
reports VR-0001 to VR-0007 in [`record/verification/`](../../record/verification/README.md)
document them in full, with their scripts and raw data.

| Experiment | Result, 2026-11-14 |
|---|---|
| 1. Connection and scene | **Done.** The Python client 0.7.0 connected to server 0.7.0 once the sandbox save was loaded, and read the scene as the Space Center. A plain call took about 0.26 ms there. |
| 2. Tick hold and rate | **Done in part.** `hold_tick` with `next_tick` stepped exactly 0.02 s of game time per tick, so 50 ticks per second, at about 19.9 ms of wall time per held cycle at the Space Center. Its cost with vessels in flight is still to be measured. |
| 3. Stepping rate | **Done for one vessel on the pad.** A held cycle of 15 calls, reading telemetry and writing the controls, took a median of 19.7 ms, 26.9 ms at the 95th percentile and 49.6 ms at most, against the game's 20 ms tick. Several vessels, and vessels in flight, are still to be measured. |
| 4. Extravehicular activity | **Done.** `CrewMember.eva` put a kerbal on the pad. The kerbal's `evaReport` and `surfaceSample` experiments ran through `Experiment.run`, and boarding again with `Control.board` stored both results in the pod's container. kRPC has no call to plant a flag, but the kerbal module's `PlantFlag` event plants one. It becomes active once the kerbal has released the ladder with `Control.release_ladder`, stands clear and carries a flag, and the save file then holds a landed flag. Reading `available` on the surface-feature experiment `ROCScience` raises a null reference inside kRPC, which is a defect in kRPC. |
| 5. Facility gates | Open. Needs the career save, read only, or a career-mode test. |
| 6. Funds reasons | Open. Needs the career save. |
| 7. Inventory | **Done.** The game reports 17 bodies and 145 biomes. Combined with the experiments' masks, the wiki's multipliers and the Space Center's 37 location biomes, that gives about 3,100 science subjects, worth about 218,000 science on Hard. Without the 34 surface features that is about 361,500 on Normal, 92 percent of the wiki's 392,019, and the gap lies in this count's assumptions. Asteroid and comet samples, deployed experiments and the laboratory are left out. |
| 8. Keleusma timing | **Done.** A three-axis attitude loop in fixed point, `crates/ikria-harness/src/attitude.rs`, stepped 200,000 times in a release build on this machine with no mismatch against its Rust reference. A step took a median of 2.29 µs, 2.71 µs at the 99th percentile and 41.5 µs at the 99.9th, with a worst case of 1.84 ms that is most likely the operating system *(inference)*. The verifier bounds an iteration at 67 nominal cycles. About 8,700 such steps fit a 20 ms tick at the median, so Keleusma is far from the bottleneck, which is the bridge. |
| 9. Physics range after separation | **Done in part, 2026-11-14.** An uncrewed pod flew on its Flea as a non-active vessel while its kerbal stood on the ground as the active one. It climbed to 12.8 km, up to 12.7 km from the kerbal, stayed loaded and off rails throughout, and was destroyed on impact rather than removed in flight. The default range kRPC reported before launch, 350 m, does not govern flight in the atmosphere. Distances beyond 13 km, and a raised `physics_range`, need a separated stage that still flies more than about 20 km from the active vessel. An uncrewed stock rocket without a probe core tipped over at 230 m, so this moves to the booster's development step V3. Added for [`VTHL_PROGRAM.md`](VTHL_PROGRAM.md). |
| 10. Two vessels in lockstep | **Done.** A held cycle reading and commanding two vessels, 16 calls in all, took a median of 18.8 ms, 31.3 ms at the 95th percentile and 43.3 ms at most, against the 20 ms tick. Added for the booster program. |

**Findings that change the design, 2026-11-14.**
- **Staging needs the active vessel.** kRPC refused `activate_next_stage` on a vessel that
  was not active. Lighting the engine through `Engine.active` on that vessel worked. A
  separated booster is therefore worked through its parts, such as engines, decouplers,
  parachutes and gear, not through staging.
- **kRPC's vessel list is not the fleet.** `SpaceCenter.vessels` left out kerbals outside a
  vessel, flags and asteroids, which the save file listed. It did list debris. The fleet and object registry is
  therefore built from the save file, with kRPC for the vessels it reports.
- **Science on Hard.** A subject's cap is its base value times the situation multiplier
  times 0.6, as an extravehicular report from the pad showed at 1.44.

**How a vessel reaches the sandbox for experiments 3 and 4** depends on open question 6
of [`OPEN_QUESTIONS.md`](OPEN_QUESTIONS.md), how an agent builds a craft. The simplest
first step is a small craft file written by hand, such as a command pod alone, launched
in the sandbox through kRPC.

## Decisions

| Question | Ruling, 2026-11-14 |
|---|---|
| D1, process layout | Three processes, the bridge, the harness and the controller. |
| D2, the bridge's home | A separate GPL-3.0 repository, using `krpc-client`. |
