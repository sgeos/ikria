//! A representative attitude-hold control loop, written in Keleusma.
//!
//! This is the loop of Phase 0 experiment 8. It exists to measure what one control step
//! costs on this machine and what bound Keleusma's verifier gives it, before the harness
//! is built around many such loops. Its arithmetic is a proportional-derivative law on
//! three axes, clamped to the controls' range.
//!
//! # Units
//!
//! Every quantity is a fixed-point integer in thousandths, so that `1000` is one control
//! unit or one radian. Keleusma's float operations must match overflow, underflow and
//! not-a-number on every operation, which would hide the cost of the control law behind
//! that matching. Fixed point keeps the measurement about the law itself. Whether the
//! campaign's loops use fixed point or floats is a later design decision.
//!
//! # Stepping
//!
//! The loop body yields one command per cycle and then resets. The first step calls the
//! entry point. Each later step resumes the virtual machine, which first completes the
//! previous cycle and resets, and is then resumed again with the same telemetry to run
//! the next cycle. [`AttitudeLoop::step`] hides that sequence.

use core::fmt;

use keleusma::compiler::compile;
use keleusma::lexer::tokenize;
use keleusma::parser::parse;
use keleusma::vm::{Vm, VmState};
use keleusma::{Arena, KeleusmaType, Value};

/// Proportional gain, applied to the attitude error.
pub const KP: i64 = 4;
/// Derivative gain, applied to the angular rate.
pub const KD: i64 = 2;
/// The magnitude of the largest command, in thousandths of the controls' full range.
pub const LIMIT: i64 = 1000;

/// The Keleusma source of the loop. Its gains and limit must match [`KP`], [`KD`] and
/// [`LIMIT`], which the tests check by comparing it with [`reference_command`].
pub const SOURCE: &str = r#"
struct Attitude {
    pitch_err: Word, yaw_err: Word, roll_err: Word,
    pitch_rate: Word, yaw_rate: Word, roll_rate: Word
}

struct Command { pitch: Word, yaw: Word, roll: Word }

fn clamp(x: Word) -> Word {
    if x > 1000 { 1000 } else { if x < 0 - 1000 { 0 - 1000 } else { x } }
}

fn pd(err: Word, rate: Word) -> Word {
    clamp(err * 4 - rate * 2)
}

loop main(a: Attitude) -> Command {
    let c = Command {
        pitch: pd(a.pitch_err, a.pitch_rate),
        yaw: pd(a.yaw_err, a.yaw_rate),
        roll: pd(a.roll_err, a.roll_rate)
    };
    let _next = yield c;
    c
}
"#;

/// The attitude error and angular rate on three axes, in thousandths of a radian and of
/// a radian per second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Attitude {
    /// Pitch error.
    pub pitch_err: i64,
    /// Yaw error.
    pub yaw_err: i64,
    /// Roll error.
    pub roll_err: i64,
    /// Pitch rate.
    pub pitch_rate: i64,
    /// Yaw rate.
    pub yaw_rate: i64,
    /// Roll rate.
    pub roll_rate: i64,
}

/// The command on three axes, in thousandths of the controls' full range. Its name and
/// fields match the Keleusma `Command` struct, which lets the virtual machine decode it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, KeleusmaType)]
pub struct Command {
    /// Pitch command.
    pub pitch: i64,
    /// Yaw command.
    pub yaw: i64,
    /// Roll command.
    pub roll: i64,
}

/// Why building or stepping the loop failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopError {
    /// The source did not lex, parse or compile.
    Build(String),
    /// The virtual machine refused the module or failed while running it.
    Vm(String),
    /// The virtual machine returned a state this loop never produces, such as finishing.
    Protocol(String),
    /// A yielded value was not a command.
    Decode(String),
}

impl fmt::Display for LoopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Build(m) => write!(f, "could not build the loop: {m}"),
            Self::Vm(m) => write!(f, "the virtual machine failed: {m}"),
            Self::Protocol(m) => write!(f, "unexpected virtual machine state: {m}"),
            Self::Decode(m) => write!(f, "could not decode the command: {m}"),
        }
    }
}

impl std::error::Error for LoopError {}

/// The command the loop must produce for an input, computed in Rust. It is the reference
/// the tests compare the Keleusma loop against.
#[must_use]
pub fn reference_command(a: &Attitude) -> Command {
    let pd = |err: i64, rate: i64| {
        (err.wrapping_mul(KP).wrapping_sub(rate.wrapping_mul(KD))).clamp(-LIMIT, LIMIT)
    };
    Command {
        pitch: pd(a.pitch_err, a.pitch_rate),
        yaw: pd(a.yaw_err, a.yaw_rate),
        roll: pd(a.roll_err, a.roll_rate),
    }
}

/// Compiles a Keleusma source into a module.
///
/// # Errors
///
/// Returns [`LoopError::Build`] if the source does not lex, parse or compile.
pub fn build(source: &str) -> Result<keleusma::bytecode::Module, LoopError> {
    let tokens = tokenize(source).map_err(|e| LoopError::Build(format!("{e:?}")))?;
    let program = parse(&tokens).map_err(|e| LoopError::Build(format!("{e:?}")))?;
    compile(&program).map_err(|e| LoopError::Build(format!("{e:?}")))
}

fn attitude_value(a: &Attitude) -> Value {
    Value::struct_value(
        "Attitude".into(),
        vec![
            ("pitch_err".into(), Value::Int(a.pitch_err)),
            ("yaw_err".into(), Value::Int(a.yaw_err)),
            ("roll_err".into(), Value::Int(a.roll_err)),
            ("pitch_rate".into(), Value::Int(a.pitch_rate)),
            ("yaw_rate".into(), Value::Int(a.yaw_rate)),
            ("roll_rate".into(), Value::Int(a.roll_rate)),
        ],
    )
}

/// The attitude loop on one virtual machine, stepped once per tick.
pub struct AttitudeLoop<'arena> {
    vm: Vm<'arena, 'arena>,
    started: bool,
}

impl<'arena> AttitudeLoop<'arena> {
    /// Builds the loop from [`SOURCE`] and verifies it over the given arena.
    ///
    /// # Errors
    ///
    /// Returns [`LoopError::Build`] if the source does not compile, or
    /// [`LoopError::Vm`] if the verifier refuses the module, for example because it
    /// cannot bound it.
    pub fn new(arena: &'arena Arena) -> Result<Self, LoopError> {
        Self::from_source(SOURCE, arena)
    }

    /// Builds a loop from any source with the same dialogue, and verifies it.
    ///
    /// # Errors
    ///
    /// As for [`AttitudeLoop::new`].
    pub fn from_source(source: &str, arena: &'arena Arena) -> Result<Self, LoopError> {
        let module = build(source)?;
        let vm = Vm::new(module, arena).map_err(|e| LoopError::Vm(format!("{e:?}")))?;
        Ok(Self { vm, started: false })
    }

    /// The verifier's bound on one iteration, in Keleusma's nominal cycles.
    ///
    /// # Errors
    ///
    /// Returns [`LoopError::Vm`] if the bound cannot be computed.
    pub fn wcet_per_iteration(&self) -> Result<u32, LoopError> {
        self.vm
            .wcet_per_iteration()
            .map_err(|e| LoopError::Vm(format!("{e:?}")))
    }

    /// Runs one control step and returns its command.
    ///
    /// # Errors
    ///
    /// Returns [`LoopError::Vm`] if the virtual machine fails, [`LoopError::Protocol`] if
    /// it returns a state other than a yield where one is due, or [`LoopError::Decode`]
    /// if the yielded value is not a command.
    pub fn step(&mut self, input: &Attitude) -> Result<Command, LoopError> {
        let value = attitude_value(input);
        let vm_err = |e: keleusma::vm::VmError| LoopError::Vm(format!("{e:?}"));
        let state = if self.started {
            match self.vm.resume(value.clone()).map_err(vm_err)? {
                VmState::Reset => self.vm.resume(value).map_err(vm_err)?,
                other => other,
            }
        } else {
            self.started = true;
            self.vm.call(&[value]).map_err(vm_err)?
        };
        // A yielded composite lives in the arena until the next resume, so it is decoded
        // here, before this function returns and any later step resumes the machine.
        match state {
            VmState::Yielded(v) => self
                .vm
                .decode::<Command>(&v)
                .map_err(|e| LoopError::Decode(format!("{e:?}"))),
            other => Err(LoopError::Protocol(format!("{other:?}"))),
        }
    }
}
