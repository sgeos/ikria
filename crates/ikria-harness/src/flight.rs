//! Verified flight dialogue and the first hop program. No game API dependencies.
//!
//! Programs share immutable dialogue declarations. Entry signatures are checked before
//! compilation, nominal cycle admission happens before arming, and any runtime fault
//! permanently poisons that virtual machine. Reset must occur exactly once per tick.
use crate::attitude::{LoopError, build};
use ikria_records::flight::Telemetry;
use ikria_records::{Axis, Fraction, PartId, Request, Situation, VesselCommand};
use keleusma::vm::{Vm, VmState};
use keleusma::{Arena, KeleusmaType, Value};

/// Canonical version 1 flight-program types.
pub const DIALOGUE: &str = include_str!("../../../programs/dialogue.kl");
/// First hop and parachute program.
pub const HOP: &str = include_str!("../../../programs/hop.kl");
/// Maximum source bytes admitted before compilation.
pub const MAX_SOURCE: usize = 65_536;

/// Decoded output before policy validation. Integer controls use thousandths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, KeleusmaType)]
pub struct FlightOutput {
    /// Next phase, in 0 through 5.
    pub phase: i64,
    /// One-shot ignition request.
    pub ignite: i64,
    /// One-shot deployment request.
    pub deploy: i64,
    /// Throttle in 0 through 1000.
    pub throttle: i64,
    /// Pitch in -1000 through 1000.
    pub pitch: i64,
    /// Yaw in -1000 through 1000.
    pub yaw: i64,
    /// Roll in -1000 through 1000.
    pub roll: i64,
}

impl FlightOutput {
    /// Refuse any output outside the canonical dialogue, before authority masking.
    pub fn validate(&self) -> Result<(), LoopError> {
        if !(0..=5).contains(&self.phase)
            || !(0..=1).contains(&self.ignite)
            || !(0..=1).contains(&self.deploy)
            || !(0..=1000).contains(&self.throttle)
            || [self.pitch, self.yaw, self.roll]
                .iter()
                .any(|x| !(-1000..=1000).contains(x))
        {
            return Err(LoopError::Protocol("output outside dialogue bounds".into()));
        }
        Ok(())
    }
}

/// Program instance confined to its constructing thread.
pub struct FlightProgram<'a> {
    vm: Vm<'a, 'a>,
    started: bool,
    failed: bool,
    cycles: u32,
}

impl<'a> FlightProgram<'a> {
    /// Compile and verify a body against the canonical dialogue and nominal cycle budget.
    /// The budget is not a calibrated wall-clock guarantee.
    pub fn new(body: &str, arena: &'a Arena, budget: u32) -> Result<Self, LoopError> {
        if body.len() > MAX_SOURCE || budget == 0 {
            return Err(LoopError::Build(
                "source or cycle budget out of bounds".into(),
            ));
        }
        let source = format!("{DIALOGUE}\n{body}");
        let ast = keleusma::parser::parse(
            &keleusma::lexer::tokenize(&source).map_err(|e| LoopError::Build(format!("{e:?}")))?,
        )
        .map_err(|e| LoopError::Build(format!("{e:?}")))?;
        let main = ast
            .functions
            .iter()
            .filter(|f| f.name == "main")
            .collect::<Vec<_>>();
        let named = |ty: &keleusma::ast::TypeExpr, expected: &str| {
            matches!(ty,
            keleusma::ast::TypeExpr::Named(name,types,consts,_) if name==expected && types.is_empty() && consts.is_empty())
        };
        if !ast.uses.is_empty()
            || !ast.data_decls.is_empty()
            || main.len() != 1
            || main[0].category != keleusma::ast::FunctionCategory::Loop
            || main[0].params.len() != 1
            || !main[0].params[0]
                .type_expr
                .as_ref()
                .is_some_and(|ty| named(ty, "FlightInput"))
            || !named(&main[0].return_type, "FlightOutput")
            || main[0].guard.is_some()
            || !main[0].type_params.is_empty()
            || !main[0].const_params.is_empty()
            || ast.types.len() != 2
        {
            return Err(LoopError::Build(
                "flight dialogue mismatch or unsupported persistent/native state".into(),
            ));
        }
        let mut module = build(&source)?;
        // Recompute bounds rather than trusting any producer declaration.
        module.wcet_cycles = 0;
        module.wcmu_bytes = 0;
        let vm = Vm::new(module, arena).map_err(|e| LoopError::Vm(format!("{e:?}")))?;
        let cycles = vm
            .wcet_per_iteration()
            .map_err(|e| LoopError::Vm(format!("{e:?}")))?;
        if cycles > budget {
            return Err(LoopError::Build(format!(
                "cycle bound {cycles} exceeds {budget}"
            )));
        }
        Ok(Self {
            vm,
            started: false,
            failed: false,
            cycles,
        })
    }

    /// Verified nominal cycle bound.
    pub fn cycles(&self) -> u32 {
        self.cycles
    }

    /// Step once. A failed instance cannot be resumed.
    pub fn step(
        &mut self,
        t: &Telemetry,
        phase: i64,
        abort: bool,
    ) -> Result<FlightOutput, LoopError> {
        if self.failed {
            return Err(LoopError::Protocol("program is faulted".into()));
        }
        self.failed = true;
        t.validate()
            .map_err(|e| LoopError::Protocol(e.to_string()))?;
        if !(0..=5).contains(&phase) {
            return Err(LoopError::Protocol("invalid phase".into()));
        }
        let [right, forward, down] = t.up_body.components();
        let [rx, ry, rz] = t.rates_body.components();
        // Vessel frame is right, forward, down. atan2 avoids vertical Euler singularity.
        // Positive pitch raises the nose toward -down, positive yaw toward +right.
        let q = |v: f64| (v * 1000.0).round() as i64;
        let fields = [
            ("phase", phase),
            ("altitude", q(t.altitude_m)),
            ("vertical_speed", q(t.vertical_speed_m_s)),
            ("fuel", q(t.solid_fuel)),
            ("safe", i64::from(t.chute_safe)),
            ("deployed", i64::from(t.chute_deployed)),
            (
                "landed",
                i64::from(matches!(
                    t.situation,
                    Situation::Landed | Situation::Splashed
                )),
            ),
            ("abort", i64::from(abort)),
            ("pitch_error", q((-down).atan2(forward))),
            ("yaw_error", q(right.atan2(forward))),
            ("pitch_rate", q(-rx)),
            ("yaw_rate", q(rz)),
            ("roll_rate", q(ry)),
        ];
        let input = Value::struct_value(
            "FlightInput".into(),
            fields
                .into_iter()
                .map(|(name, v)| (name.into(), Value::Int(v)))
                .collect(),
        );
        let err = |e| LoopError::Vm(format!("{e:?}"));
        let state = if self.started {
            match self.vm.resume(input.clone()).map_err(err)? {
                VmState::Reset => self.vm.resume(input).map_err(err)?,
                other => {
                    return Err(LoopError::Protocol(format!(
                        "expected Reset, got {other:?}"
                    )));
                }
            }
        } else {
            self.started = true;
            self.vm.call(&[input]).map_err(err)?
        };
        let output = match state {
            VmState::Yielded(v) => self
                .vm
                .decode::<FlightOutput>(&v)
                .map_err(|e| LoopError::Decode(format!("{e:?}")))?,
            other => {
                return Err(LoopError::Protocol(format!(
                    "expected Yielded, got {other:?}"
                )));
            }
        };
        output.validate()?;
        self.failed = false;
        Ok(output)
    }
}

/// Convert a program output into a command. Part-level instrumentation is restricted
/// to verification. No-control capsules receive no continuous flight control.
pub fn command(
    t: &Telemetry,
    output: FlightOutput,
    engine: PartId,
    chute: PartId,
) -> Result<VesselCommand, LoopError> {
    output.validate()?;
    t.validate()
        .map_err(|e| LoopError::Protocol(e.to_string()))?;
    let mut c = VesselCommand {
        id: t.id,
        ..VesselCommand::default()
    };
    let err = |e: ikria_records::RecordError| LoopError::Protocol(e.to_string());
    if t.controllable {
        c.throttle = Fraction::new(output.throttle as f64 / 1000.0).map_err(err)?;
        c.pitch = Axis::new(output.pitch as f64 / 1000.0).map_err(err)?;
        c.yaw = Axis::new(output.yaw as f64 / 1000.0).map_err(err)?;
        c.roll = Axis::new(output.roll as f64 / 1000.0).map_err(err)?;
    }
    if output.ignite == 1 {
        c.requests.push(Request::ActivateEngine { part: engine });
    }
    if output.deploy == 1 {
        c.requests.push(Request::DeployParachute { part: chute });
    }
    Ok(c)
}
