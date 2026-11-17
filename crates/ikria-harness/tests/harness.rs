//! Deterministic scheduler tests without sockets, wall time or the game.
//!
//! Keleusma outputs are compared with the independent Rust control law. Fault tests
//! count component calls to verify that refusal and terminal states have no retries.

use std::{cell::Cell, rc::Rc};

use ikria_harness::{
    attitude::{Attitude, AttitudeLoop, LIMIT, LoopError, SOURCE, reference_command},
    control::{ControlLoop, RateDamper},
    harness::{Harness, HarnessError},
    host::{AppliedCommand, Host, ScriptError, ScriptedHost, Tick},
};
use ikria_records::{
    Axis, GameTime, Quaternion, Request, Situation, Vector3, VesselCommand, VesselId,
    VesselTelemetry,
};
use keleusma::{Arena, vm::DEFAULT_ARENA_CAPACITY};

const VESSEL: VesselId = VesselId(7);

fn tick(seconds: f64, rates: [f64; 3]) -> Tick {
    Tick {
        ut: GameTime::new(seconds).unwrap(),
        vessel: VesselTelemetry {
            id: VESSEL,
            situation: Situation::Flying,
            body: "Kerbin".into(),
            position: Vector3::new(600_000.0, 0.0, 0.0).unwrap(),
            velocity: Vector3::new(0.0, 0.0, 0.0).unwrap(),
            rotation: Quaternion::new([0.0, 0.0, 0.0, 1.0]).unwrap(),
            angular_velocity: Vector3::try_from(rates).unwrap(),
            mass_kg: 1_000.0,
            available_thrust_n: 0.0,
            altitude_surface_m: 100.0,
            altitude_sea_level_m: 170.0,
            dynamic_pressure_pa: 0.0,
            stage: 1,
            has_link: false,
            resources: vec![],
        },
    }
}

fn neutral(ut: GameTime) -> AppliedCommand {
    AppliedCommand {
        ut,
        command: VesselCommand {
            id: VESSEL,
            ..VesselCommand::default()
        },
    }
}

fn replay(ticks: Vec<Tick>) -> Vec<AppliedCommand> {
    let count = ticks.len();
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let mut harness = Harness::new(
        ScriptedHost::new(ticks),
        VESSEL,
        RateDamper::new(&arena).unwrap(),
    );
    for _ in 0..count {
        assert!(harness.step().unwrap().is_some());
    }
    assert_eq!(harness.step().unwrap(), None);
    harness.into_host().applied().to_vec()
}

#[test]
fn consecutive_keleusma_ticks_match_the_reference_and_replay_identically() {
    let rates = [-1.5, -0.5, -0.001, 0.0, 0.001, 0.125, 0.5, 1.5];
    let ticks: Vec<_> = rates
        .into_iter()
        .enumerate()
        .map(|(i, rate)| tick(i as f64 * 0.02, [rate, -rate, rate / 2.0]))
        .collect();
    let first = replay(ticks.clone());
    assert_eq!(first, replay(ticks.clone()));
    assert_eq!(first.len(), ticks.len());
    for (applied, tick) in first.iter().zip(ticks) {
        let [x, y, z] = tick.vessel.angular_velocity.components();
        // The source inputs are exact thousandths. Applying the reference to the
        // unsaturated rates also checks the adapter's earlier saturation is harmless.
        let reference = reference_command(&Attitude {
            pitch_rate: (x * 1000.0).round() as i64,
            yaw_rate: (y * 1000.0).round() as i64,
            roll_rate: (z * 1000.0).round() as i64,
            ..Attitude::default()
        });
        assert_eq!(applied.ut, tick.ut);
        assert_eq!(
            applied.command,
            VesselCommand {
                id: VESSEL,
                pitch: Axis::new(reference.pitch as f64 / LIMIT as f64).unwrap(),
                yaw: Axis::new(reference.yaw as f64 / LIMIT as f64).unwrap(),
                roll: Axis::new(reference.roll as f64 / LIMIT as f64).unwrap(),
                ..VesselCommand::default()
            }
        );
    }
}

#[test]
fn rate_conversion_rounds_half_units_and_saturates_extreme_finite_values() {
    let applied = replay(vec![
        tick(0.0, [0.0005, -0.0005, 0.00049]),
        tick(0.02, [f64::MAX, f64::MIN, 0.0]),
    ]);
    let axes = |c: &AppliedCommand| {
        [
            c.command.pitch.value(),
            c.command.yaw.value(),
            c.command.roll.value(),
        ]
    };
    assert_eq!(axes(&applied[0]), [-0.002, 0.002, 0.0]);
    assert_eq!(axes(&applied[1]), [-1.0, 1.0, 0.0]);
}

#[test]
fn a_script_cannot_advance_until_the_command_is_applied() {
    let first = tick(0.0, [0.0; 3]);
    let second = tick(0.02, [0.0; 3]);
    let mut host = ScriptedHost::new(vec![first.clone(), second.clone()]);
    assert_eq!(host.next_tick().unwrap(), Some(first.clone()));
    assert_eq!(host.next_tick(), Err(ScriptError::TickHeld));
    assert!(host.applied().is_empty());
    let command = neutral(first.ut);
    host.apply_command(command.clone()).unwrap();
    assert_eq!(host.applied(), &[command]);
    assert_eq!(host.next_tick().unwrap(), Some(second.clone()));
    host.apply_command(neutral(second.ut)).unwrap();
    assert_eq!(host.next_tick().unwrap(), None);
}

#[test]
fn a_script_refuses_commands_without_a_held_tick() {
    let mut host = ScriptedHost::new(vec![]);
    assert_eq!(
        host.apply_command(neutral(GameTime::new(0.0).unwrap())),
        Err(ScriptError::NoHeldTick)
    );
    assert_eq!(host.next_tick().unwrap(), None);
    assert!(host.applied().is_empty());
}

#[test]
fn a_script_refuses_bad_commands_without_losing_the_held_tick() {
    let first = tick(0.0, [0.0; 3]);
    let mut host = ScriptedHost::new(vec![first.clone()]);
    host.next_tick().unwrap();
    let mut command = neutral(first.ut);
    command.ut = GameTime::new(0.02).unwrap();
    assert_eq!(host.apply_command(command), Err(ScriptError::WrongTime));
    let mut command = neutral(first.ut);
    command.command.id = VesselId(99);
    assert_eq!(host.apply_command(command), Err(ScriptError::WrongVessel));
    let mut command = neutral(first.ut);
    command
        .command
        .requests
        .push(Request::ActionGroup { group: 0 });
    assert!(matches!(
        host.apply_command(command),
        Err(ScriptError::InvalidCommand(_))
    ));
    assert!(host.applied().is_empty());
    assert_eq!(host.next_tick(), Err(ScriptError::TickHeld));
    host.apply_command(neutral(first.ut)).unwrap();
    assert_eq!(host.applied().len(), 1);
    assert_eq!(host.next_tick().unwrap(), None);
}

struct CountingLoop {
    calls: Rc<Cell<usize>>,
    command: VesselCommand,
}

impl ControlLoop for CountingLoop {
    fn step(&mut self, _: &VesselTelemetry) -> Result<VesselCommand, LoopError> {
        self.calls.set(self.calls.get() + 1);
        Ok(self.command.clone())
    }
}

fn counting_loop() -> (CountingLoop, Rc<Cell<usize>>) {
    let calls = Rc::new(Cell::new(0));
    (
        CountingLoop {
            calls: Rc::clone(&calls),
            command: neutral(GameTime::new(0.0).unwrap()).command,
        },
        calls,
    )
}

#[test]
fn a_wrong_vessel_is_refused_before_the_loop_runs() {
    let (control, calls) = counting_loop();
    let mut wrong = tick(0.0, [0.0; 3]);
    wrong.vessel.id = VesselId(99);
    let mut harness = Harness::new(ScriptedHost::new(vec![wrong]), VESSEL, control);
    assert!(matches!(harness.step(), Err(HarnessError::WrongVessel)));
    assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
    assert_eq!(calls.get(), 0);
    assert!(harness.host().applied().is_empty());
}

#[test]
fn repeated_and_backwards_time_stop_before_another_control_step() {
    for second in [0.02, 0.0] {
        let (control, calls) = counting_loop();
        let mut harness = Harness::new(
            ScriptedHost::new(vec![tick(0.02, [0.0; 3]), tick(second, [0.0; 3])]),
            VESSEL,
            control,
        );
        assert!(harness.step().unwrap().is_some());
        assert!(matches!(
            harness.step(),
            Err(HarnessError::NonIncreasingTime)
        ));
        assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
        assert_eq!(calls.get(), 1);
        assert_eq!(harness.host().applied().len(), 1);
    }
}

#[test]
fn a_command_for_another_vessel_is_never_applied() {
    let (mut control, calls) = counting_loop();
    control.command.id = VesselId(99);
    let mut harness = Harness::new(
        ScriptedHost::new(vec![tick(0.0, [0.0; 3])]),
        VESSEL,
        control,
    );
    assert!(matches!(
        harness.step(),
        Err(HarnessError::WrongCommandVessel)
    ));
    assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
    assert_eq!(calls.get(), 1);
    assert!(harness.host().applied().is_empty());
}

#[test]
fn an_invalid_request_is_never_applied() {
    let (mut control, calls) = counting_loop();
    control
        .command
        .requests
        .push(Request::ActionGroup { group: 11 });
    let mut harness = Harness::new(
        ScriptedHost::new(vec![tick(0.0, [0.0; 3])]),
        VESSEL,
        control,
    );
    assert!(matches!(
        harness.step(),
        Err(HarnessError::InvalidCommand(_))
    ));
    assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
    assert_eq!(calls.get(), 1);
    assert!(harness.host().applied().is_empty());
}

#[test]
fn a_keleusma_runtime_fault_stops_without_applying_a_command() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let source = SOURCE.replace("clamp(err * 4 - rate * 2)", "err / rate");
    let control = RateDamper::from_loop(AttitudeLoop::from_source(&source, &arena).unwrap());
    let mut harness = Harness::new(
        ScriptedHost::new(vec![tick(0.0, [0.0; 3])]),
        VESSEL,
        control,
    );
    assert!(matches!(
        harness.step(),
        Err(HarnessError::Loop(LoopError::Vm(_)))
    ));
    assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
    assert!(harness.host().applied().is_empty());
}

#[test]
fn a_keleusma_output_outside_the_control_range_is_refused() {
    let arena = Arena::with_capacity(DEFAULT_ARENA_CAPACITY);
    let source = SOURCE.replace("clamp(err * 4 - rate * 2)", "1001");
    let control = RateDamper::from_loop(AttitudeLoop::from_source(&source, &arena).unwrap());
    let mut harness = Harness::new(
        ScriptedHost::new(vec![tick(0.0, [0.0; 3])]),
        VESSEL,
        control,
    );
    assert!(matches!(
        harness.step(),
        Err(HarnessError::Loop(LoopError::Decode(_)))
    ));
    assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
    assert!(harness.host().applied().is_empty());
}

struct ProbeHost {
    reads: Rc<Cell<usize>>,
    writes: Rc<Cell<usize>>,
    fail_read: bool,
    tick: Option<Tick>,
}

impl Host for ProbeHost {
    type Error = std::io::Error;

    fn next_tick(&mut self) -> Result<Option<Tick>, Self::Error> {
        self.reads.set(self.reads.get() + 1);
        if self.fail_read {
            return Err(std::io::Error::other("injected read failure"));
        }
        Ok(self.tick.take())
    }

    fn apply_command(&mut self, _: AppliedCommand) -> Result<(), Self::Error> {
        self.writes.set(self.writes.get() + 1);
        Err(std::io::Error::other("injected write failure"))
    }
}

#[test]
fn host_read_and_write_failures_are_terminal_and_never_retried() {
    for fail_read in [true, false] {
        let reads = Rc::new(Cell::new(0));
        let writes = Rc::new(Cell::new(0));
        let (control, calls) = counting_loop();
        let mut harness = Harness::new(
            ProbeHost {
                reads: Rc::clone(&reads),
                writes: Rc::clone(&writes),
                fail_read,
                tick: Some(tick(0.0, [0.0; 3])),
            },
            VESSEL,
            control,
        );
        assert!(matches!(harness.step(), Err(HarnessError::Host(_))));
        assert!(matches!(harness.step(), Err(HarnessError::Stopped)));
        assert_eq!(reads.get(), 1);
        assert_eq!(writes.get(), usize::from(!fail_read));
        assert_eq!(calls.get(), usize::from(!fail_read));
    }
}

#[test]
fn exhaustion_is_terminal_without_touching_the_host_or_loop_again() {
    let reads = Rc::new(Cell::new(0));
    let writes = Rc::new(Cell::new(0));
    let (control, calls) = counting_loop();
    let mut harness = Harness::new(
        ProbeHost {
            reads: Rc::clone(&reads),
            writes: Rc::clone(&writes),
            fail_read: false,
            tick: None,
        },
        VESSEL,
        control,
    );
    assert_eq!(harness.step().unwrap(), None);
    assert_eq!(harness.step().unwrap(), None);
    assert_eq!(reads.get(), 1);
    assert_eq!(writes.get(), 0);
    assert_eq!(calls.get(), 0);
}
