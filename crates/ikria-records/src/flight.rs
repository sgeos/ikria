//! Bounded, fail-closed dialogue for a single verification flight.
//!
//! Version 2 is deliberately separate from the version 1 scripted experiment.
//! Identifiers are session-local. Every connection has one vessel and one outstanding
//! sample. Acknowledgement means the command calls returned, not that physics achieved
//! the requested result. A lost acknowledgement must never trigger a retry.
use crate::{GameTime, PartId, RecordError, Situation, Vector3, VesselCommand, VesselId};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

/// Live dialogue version.
pub const VERSION: u32 = 2;
/// Maximum encoded message bytes, including newline.
pub const MAX_FRAME: usize = 65_536;
/// This release permits mutations only in this verification save.
pub const VERIFICATION_SAVE: &str = "Ikria Physical Verification";

/// State required by the first flight programs. Angles are never Euler angles.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Telemetry {
    /// Session-local vessel identity.
    pub id: VesselId,
    /// Situation.
    pub situation: Situation,
    /// Height above terrain in metres.
    pub altitude_m: f64,
    /// Surface vertical velocity in metres per second, positive upwards.
    pub vertical_speed_m_s: f64,
    /// Remaining solid fuel in resource units.
    pub solid_fuel: f64,
    /// Total mass in kilograms.
    pub mass_kg: f64,
    /// Crew aboard. Verification flights require zero.
    pub crew: u32,
    /// Full control authority as reported by the host.
    pub controllable: bool,
    /// Ground connection. Onboard programs do not require it.
    pub has_link: bool,
    /// Loaded and off rails.
    pub physics_active: bool,
    /// Number of parts. A change terminates this single-body development profile.
    pub parts: u32,
    /// Engine is active.
    pub engine_active: bool,
    /// Parachute has been deployed.
    pub chute_deployed: bool,
    /// Host reports deployment safe.
    pub chute_safe: bool,
    /// Local up in vessel coordinates, right, forward, down. Unit vector.
    pub up_body: Vector3,
    /// Angular velocity in vessel coordinates, right, forward, down, rad/s.
    pub rates_body: Vector3,
}

impl Telemetry {
    /// Validate scalar bounds and the direction norm before any loop sees a sample.
    pub fn validate(&self) -> Result<(), RecordError> {
        crate::in_range("altitude", self.altitude_m, -1000.0, 1e9)?;
        crate::in_range("vertical speed", self.vertical_speed_m_s, -1e6, 1e6)?;
        crate::in_range("solid fuel", self.solid_fuel, 0.0, 1e9)?;
        crate::in_range("mass", self.mass_kg, 0.001, 1e12)?;
        crate::in_range("parts", f64::from(self.parts), 1.0, 10000.0)?;
        let norm = self
            .up_body
            .components()
            .iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt();
        crate::in_range("up direction norm", norm, 0.999, 1.001)?;
        for rate in self.rates_body.components() {
            crate::in_range("angular rate", rate, -1000.0, 1000.0)?;
        }
        Ok(())
    }
}

/// Bridge to harness messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum BridgeFrame {
    /// Identity and capability declaration, before any held tick.
    Hello {
        /// Dialogue version.
        version: u32,
        /// Actual loaded save, checked by a unique snapshot.
        save: String,
        /// Vessel name.
        vessel: String,
        /// Whether this session may apply commands.
        armed: bool,
        /// Session-local engine part identifier.
        engine: PartId,
        /// Session-local parachute part identifier.
        chute: PartId,
    },
    /// One held tick.
    Sample {
        /// Strictly consecutive sequence, starting at zero.
        seq: u64,
        /// Game clock.
        ut: GameTime,
        /// State of the bound vessel.
        telemetry: Telemetry,
    },
    /// All command calls completed successfully.
    Applied {
        /// Sequence answered.
        seq: u64,
    },
    /// Session is terminal. No retry is permitted.
    Failed {
        /// Diagnostic, not an instruction.
        reason: String,
    },
}

/// Harness to bridge messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum HarnessFrame {
    /// Version agreement.
    Hello {
        /// Dialogue version.
        version: u32,
    },
    /// Answer exactly one sample.
    Command {
        /// Sequence being answered.
        seq: u64,
        /// Exact sample time.
        ut: GameTime,
        /// Validated command.
        command: VesselCommand,
    },
    /// Neutralize continuous controls and release the tick. Does not stop a solid motor.
    Close {},
}

/// Read one bounded, newline-terminated frame. EOF and truncated frames are errors.
pub fn read_frame<R: BufRead, T: serde::de::DeserializeOwned>(
    reader: &mut R,
) -> Result<T, RecordError> {
    let mut bytes = Vec::new();
    // take bounds allocation even when a peer never terminates its line.
    std::io::Read::take(reader, MAX_FRAME as u64)
        .read_until(b'\n', &mut bytes)
        .map_err(|e| RecordError::Codec(e.to_string()))?;
    if bytes.last() != Some(&b'\n') || bytes.len() >= MAX_FRAME {
        return Err(RecordError::Codec("truncated or oversized frame".into()));
    }
    serde_json::from_slice(&bytes).map_err(|e| RecordError::Codec(e.to_string()))
}

/// Write and flush one bounded frame.
pub fn write_frame<W: Write, T: Serialize>(writer: &mut W, frame: &T) -> Result<(), RecordError> {
    let mut bytes = serde_json::to_vec(frame).map_err(|e| RecordError::Codec(e.to_string()))?;
    if bytes.len() + 1 >= MAX_FRAME {
        return Err(RecordError::Codec("oversized frame".into()));
    }
    bytes.push(b'\n');
    writer
        .write_all(&bytes)
        .and_then(|()| writer.flush())
        .map_err(|e| RecordError::Codec(e.to_string()))
}

/// Authorize the entire command before any side effect. This development profile
/// permits only ignition on the pad and safe parachute deployment after burnout.
/// It is not a career qualification or a general spacecraft policy.
pub fn authorize(
    t: &Telemetry,
    c: &VesselCommand,
    engine: PartId,
    chute: PartId,
    armed: bool,
) -> Result<(), RecordError> {
    t.validate()?;
    let refuse = |reason: &str| Err(RecordError::Codec(reason.into()));
    if t.crew != 0 || !t.physics_active || c.id != t.id {
        return refuse("crew, physics or vessel identity forbids command");
    }
    let continuous = c.throttle.value() != 0.0
        || [
            c.pitch,
            c.yaw,
            c.roll,
            c.forward,
            c.right,
            c.up,
            c.wheel_throttle,
            c.wheel_steering,
        ]
        .iter()
        .any(|x| x.value() != 0.0);
    if (!armed && (continuous || !c.requests.is_empty())) || (!t.controllable && continuous) {
        return refuse("session or vessel lacks control authority");
    }
    // This profile does not use translation or wheels.
    if [c.forward, c.right, c.up, c.wheel_throttle, c.wheel_steering]
        .iter()
        .any(|x| x.value() != 0.0)
        || c.requests.len() > 1
    {
        return refuse("command outside hop capabilities");
    }
    for r in &c.requests {
        let allowed = match r {
            crate::Request::ActivateEngine { part } => {
                *part == engine
                    && t.situation == Situation::PreLaunch
                    && !t.engine_active
                    && t.solid_fuel > 0.0
                    && !t.chute_deployed
            }
            crate::Request::DeployParachute { part } => {
                *part == chute
                    && !t.chute_deployed
                    && t.chute_safe
                    && t.solid_fuel < 0.01
                    && t.vertical_speed_m_s < -1.0
                    && t.altitude_m > 5.0
                    && t.altitude_m < 1500.0
            }
            _ => false,
        };
        if !allowed {
            return refuse("discrete request outside verified profile envelope");
        }
    }
    Ok(())
}

/// One-shot discrete actions already commanded in this session.
///
/// Telemetry flags such as `chute_deployed` can lag the command that changes them by
/// several ticks. A command whose intent has been journalled is treated as done from
/// then on, whether or not it was acknowledged, because a missing acknowledgement
/// leaves its outcome uncertain and an uncertain command is never repeated. The latch
/// only ever adds refusals. It never permits a command that [`authorize`] refuses.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Commanded {
    /// An engine ignition has been commanded.
    pub ignition: bool,
    /// A parachute deployment has been commanded.
    pub deployment: bool,
}

impl Commanded {
    /// Record the discrete requests of a command about to be sent. Call this before
    /// the command is written, so that a failed write still latches the action.
    pub fn record(&mut self, c: &VesselCommand) {
        for r in &c.requests {
            match r {
                crate::Request::ActivateEngine { .. } => self.ignition = true,
                crate::Request::DeployParachute { .. } => self.deployment = true,
                _ => {}
            }
        }
    }

    /// The sample as programs and policy see it. A commanded action counts as done
    /// even while the telemetry flag still lags. The journal keeps the raw sample.
    pub fn latch(&self, t: &Telemetry) -> Telemetry {
        let mut view = t.clone();
        view.engine_active |= self.ignition;
        view.chute_deployed |= self.deployment;
        view
    }
}

/// [`authorize`] with the session's one-shot latch. A repeated ignition or deployment
/// is refused even when the telemetry flag that would refuse it has not yet changed.
/// The bridge may keep calling [`authorize`], whose signature and behaviour are unchanged.
pub fn authorize_latched(
    t: &Telemetry,
    c: &VesselCommand,
    engine: PartId,
    chute: PartId,
    armed: bool,
    commanded: Commanded,
) -> Result<(), RecordError> {
    for r in &c.requests {
        let repeated = match r {
            crate::Request::ActivateEngine { .. } => commanded.ignition,
            crate::Request::DeployParachute { .. } => commanded.deployment,
            _ => false,
        };
        if repeated {
            return Err(RecordError::Codec(
                "one-shot discrete request already commanded".into(),
            ));
        }
    }
    authorize(&commanded.latch(t), c, engine, chute, armed)
}
