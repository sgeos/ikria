//! The scheduler's control-loop boundary and a scripted rate-damping adapter.

use ikria_records::{Axis, VesselCommand, VesselTelemetry};
use keleusma::Arena;

use crate::attitude::{Attitude, AttitudeLoop, LIMIT, LoopError};

/// A single vessel's control program, called once for each accepted tick.
///
/// The result must own its data before another call can resume the virtual machine.
pub trait ControlLoop {
    /// Computes a command for the supplied vessel.
    ///
    /// # Errors
    ///
    /// Returns a build, runtime, dialogue or decoding failure. A failing loop must not
    /// be resumed by the scheduler.
    fn step(&mut self, telemetry: &VesselTelemetry) -> Result<VesselCommand, LoopError>;
}

/// A demonstration adapter around the verified Keleusma attitude loop.
///
/// The scripted convention maps angular velocity `[x, y, z]` to pitch, yaw and roll
/// rates with zero attitude error. Rates are rounded to thousandths of a radian per
/// second and saturated at one radian per second before conversion. With the loop's
/// derivative gain of two, all larger magnitudes already produce full control.
///
/// This exercises telemetry, execution and control conversion. It does not transform
/// reference frames or track a target orientation and is not a qualified flight program.
pub struct RateDamper<'arena> {
    loop_: AttitudeLoop<'arena>,
}

impl<'arena> RateDamper<'arena> {
    /// Builds and verifies the demonstration Keleusma loop.
    ///
    /// # Errors
    ///
    /// Returns the build or verification error from [`AttitudeLoop::new`].
    pub fn new(arena: &'arena Arena) -> Result<Self, LoopError> {
        Ok(Self::from_loop(AttitudeLoop::new(arena)?))
    }

    /// Wraps a loop with the same attitude dialogue.
    ///
    /// Custom sources must use the documented thousandths and rate convention. This
    /// constructor permits fault injection without changing the production loop.
    #[must_use]
    pub fn from_loop(loop_: AttitudeLoop<'arena>) -> Self {
        Self { loop_ }
    }
}

impl ControlLoop for RateDamper<'_> {
    fn step(&mut self, telemetry: &VesselTelemetry) -> Result<VesselCommand, LoopError> {
        let [x, y, z] = telemetry.angular_velocity.components();
        let milli = |rate: f64| (rate.clamp(-1.0, 1.0) * LIMIT as f64).round() as i64;
        let output = self.loop_.step(&Attitude {
            pitch_rate: milli(x),
            yaw_rate: milli(y),
            roll_rate: milli(z),
            ..Attitude::default()
        })?;
        let axis = |value: i64| {
            Axis::new(value as f64 / LIMIT as f64).map_err(|e| LoopError::Decode(e.to_string()))
        };
        Ok(VesselCommand {
            id: telemetry.id,
            pitch: axis(output.pitch)?,
            yaw: axis(output.yaw)?,
            roll: axis(output.roll)?,
            ..VesselCommand::default()
        })
    }
}
