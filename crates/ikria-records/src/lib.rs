//! Ikria's record types, version 1, and the messages between its processes.
//!
//! These are the types the bridge sends the harness each tick, the commands the harness
//! returns, and the handshake that opens a session. The design is
//! `docs/architecture/HARNESS_DESIGN.md`. They name nothing of kRPC, so that the harness
//! and the mission controller never depend on the game's interface.
//!
//! # Trust
//!
//! Every value that crosses a process boundary is untrusted. Each type that carries a
//! constraint, such as a finite number or a control in range, checks it in its
//! constructor and again when it is deserialized, so that an invalid value is refused at
//! the boundary rather than reaching a control loop.
//!
//! # Encoding
//!
//! Version 1 encodes each message as one line of JSON. JSON is easy to inspect, it serves
//! as the telemetry log as well, and at 50 small messages a second its cost is small. The
//! cost is to be measured once the bridge exists, and a binary encoding can replace JSON
//! behind the same types if it proves too slow.

use core::fmt;

use serde::{Deserialize, Serialize};

/// The version of the dialogue between the bridge and the harness. Both sides refuse a
/// session whose version differs from their own.
pub const PROTOCOL_VERSION: u32 = 1;

/// Why a value was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
    /// A number was not finite.
    NotFinite(&'static str),
    /// A number was outside its allowed range.
    OutOfRange {
        /// The quantity refused.
        what: &'static str,
        /// The value given, as text.
        value: String,
    },
    /// A message could not be encoded or decoded.
    Codec(String),
    /// The peer speaks another protocol version.
    VersionMismatch {
        /// This side's version.
        ours: u32,
        /// The peer's version.
        theirs: u32,
    },
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite(what) => write!(f, "{what} is not a finite number"),
            Self::OutOfRange { what, value } => write!(f, "{what} is out of range: {value}"),
            Self::Codec(m) => write!(f, "message codec: {m}"),
            Self::VersionMismatch { ours, theirs } => {
                write!(f, "protocol version {theirs} does not match ours, {ours}")
            }
        }
    }
}

impl std::error::Error for RecordError {}

fn finite(what: &'static str, v: f64) -> Result<f64, RecordError> {
    if v.is_finite() {
        Ok(v)
    } else {
        Err(RecordError::NotFinite(what))
    }
}

fn in_range(what: &'static str, v: f64, lo: f64, hi: f64) -> Result<f64, RecordError> {
    let v = finite(what, v)?;
    if (lo..=hi).contains(&v) {
        Ok(v)
    } else {
        Err(RecordError::OutOfRange {
            what,
            value: v.to_string(),
        })
    }
}

/// The game's universal time, in seconds. Finite and not negative.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct GameTime(f64);

impl GameTime {
    /// Checks and wraps a universal time.
    ///
    /// # Errors
    ///
    /// Refuses a value that is not finite or is negative.
    pub fn new(seconds: f64) -> Result<Self, RecordError> {
        in_range("game time", seconds, 0.0, f64::MAX).map(Self)
    }

    /// The time in seconds.
    #[must_use]
    pub fn seconds(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for GameTime {
    type Error = RecordError;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

impl From<GameTime> for f64 {
    fn from(t: GameTime) -> Self {
        t.0
    }
}

/// A vessel's identifier, which the bridge keeps stable for the life of the vessel.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Serialize, Deserialize,
)]
pub struct VesselId(pub u64);

/// A part's identifier within its vessel, from the game's persistent part identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartId(pub u64);

/// A vector of three finite components.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "[f64; 3]", into = "[f64; 3]")]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    /// Checks and builds a vector.
    ///
    /// # Errors
    ///
    /// Refuses a component that is not finite.
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, RecordError> {
        Ok(Self {
            x: finite("vector x", x)?,
            y: finite("vector y", y)?,
            z: finite("vector z", z)?,
        })
    }

    /// The components, as `[x, y, z]`.
    #[must_use]
    pub fn components(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl TryFrom<[f64; 3]> for Vector3 {
    type Error = RecordError;
    fn try_from(v: [f64; 3]) -> Result<Self, Self::Error> {
        Self::new(v[0], v[1], v[2])
    }
}

impl From<Vector3> for [f64; 3] {
    fn from(v: Vector3) -> Self {
        v.components()
    }
}

/// A rotation as a unit quaternion `[x, y, z, w]`, normalized to within one part in a
/// thousand.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "[f64; 4]", into = "[f64; 4]")]
pub struct Quaternion([f64; 4]);

impl Quaternion {
    /// Checks and builds a rotation.
    ///
    /// # Errors
    ///
    /// Refuses a component that is not finite, or a quaternion whose norm differs from one
    /// by more than one part in a thousand.
    pub fn new(q: [f64; 4]) -> Result<Self, RecordError> {
        for c in q {
            finite("quaternion", c)?;
        }
        let norm = q.iter().map(|c| c * c).sum::<f64>().sqrt();
        in_range("quaternion norm", norm, 0.999, 1.001)?;
        Ok(Self(q))
    }

    /// The components, as `[x, y, z, w]`.
    #[must_use]
    pub fn components(self) -> [f64; 4] {
        self.0
    }
}

impl TryFrom<[f64; 4]> for Quaternion {
    type Error = RecordError;
    fn try_from(q: [f64; 4]) -> Result<Self, Self::Error> {
        Self::new(q)
    }
}

impl From<Quaternion> for [f64; 4] {
    fn from(q: Quaternion) -> Self {
        q.0
    }
}

/// A control in `[-1, 1]`, such as pitch or steering.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Axis(f64);

impl Axis {
    /// Checks and wraps a control.
    ///
    /// # Errors
    ///
    /// Refuses a value that is not finite or lies outside `[-1, 1]`.
    pub fn new(v: f64) -> Result<Self, RecordError> {
        in_range("axis", v, -1.0, 1.0).map(Self)
    }

    /// The value.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Axis {
    type Error = RecordError;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

impl From<Axis> for f64 {
    fn from(a: Axis) -> Self {
        a.0
    }
}

/// A control in `[0, 1]`, such as throttle.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct Fraction(f64);

impl Fraction {
    /// Checks and wraps a control.
    ///
    /// # Errors
    ///
    /// Refuses a value that is not finite or lies outside `[0, 1]`.
    pub fn new(v: f64) -> Result<Self, RecordError> {
        in_range("fraction", v, 0.0, 1.0).map(Self)
    }

    /// The value.
    #[must_use]
    pub fn value(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Fraction {
    type Error = RecordError;
    fn try_from(v: f64) -> Result<Self, Self::Error> {
        Self::new(v)
    }
}

impl From<Fraction> for f64 {
    fn from(f: Fraction) -> Self {
        f.0
    }
}

/// A vessel's situation, as the game classifies it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Situation {
    /// On the launch site, before launch.
    PreLaunch,
    /// On a surface.
    Landed,
    /// On water.
    Splashed,
    /// In an atmosphere.
    Flying,
    /// On a trajectory that meets the surface or the atmosphere.
    SubOrbital,
    /// In orbit.
    Orbiting,
    /// On a trajectory leaving the body's sphere of influence.
    Escaping,
    /// Docked to another vessel.
    Docked,
}

/// The amount of one resource aboard a vessel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// The game's name for the resource, such as `LiquidFuel`.
    pub name: String,
    /// The amount aboard, in the game's units.
    pub amount: f64,
    /// The capacity aboard, in the game's units.
    pub capacity: f64,
}

/// One vessel's state at one tick, before the filter of rule R9 is applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VesselTelemetry {
    /// The vessel.
    pub id: VesselId,
    /// Its situation.
    pub situation: Situation,
    /// The body whose sphere of influence it is in.
    pub body: String,
    /// Position in the body's non-rotating frame, in metres.
    pub position: Vector3,
    /// Velocity in the body's non-rotating frame, in metres per second.
    pub velocity: Vector3,
    /// Attitude in the body's non-rotating frame.
    pub rotation: Quaternion,
    /// Angular velocity, in radians per second.
    pub angular_velocity: Vector3,
    /// Mass, in kilograms.
    pub mass_kg: f64,
    /// Thrust available from the active engines, in newtons.
    pub available_thrust_n: f64,
    /// Altitude above the terrain, in metres.
    pub altitude_surface_m: f64,
    /// Altitude above sea level, in metres.
    pub altitude_sea_level_m: f64,
    /// Dynamic pressure, in pascals.
    pub dynamic_pressure_pa: f64,
    /// The current stage.
    pub stage: i32,
    /// Whether the vessel has a link to the ground network, which rule R3 depends on.
    pub has_link: bool,
    /// Resources aboard.
    pub resources: Vec<Resource>,
}

/// A request a loop may make of its vessel, beyond the continuous controls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Request {
    /// Fire an action group, 1 to 10.
    ActionGroup {
        /// The group.
        group: u8,
    },
    /// Fire the abort action group.
    Abort,
    /// Light one engine, which works on a vessel that is not active, unlike staging.
    ActivateEngine {
        /// The engine part.
        part: PartId,
    },
    /// Fire one decoupler.
    Decouple {
        /// The decoupler part.
        part: PartId,
    },
    /// Run one experiment.
    RunExperiment {
        /// The experiment's part.
        part: PartId,
    },
}

impl Request {
    /// Checks a request's own constraints.
    ///
    /// # Errors
    ///
    /// Refuses an action group outside 1 to 10.
    pub fn validate(&self) -> Result<(), RecordError> {
        match self {
            Self::ActionGroup { group } if !(1..=10).contains(group) => {
                Err(RecordError::OutOfRange {
                    what: "action group",
                    value: group.to_string(),
                })
            }
            _ => Ok(()),
        }
    }
}

/// One vessel's commands for one tick.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct VesselCommand {
    /// The vessel.
    pub id: VesselId,
    /// Throttle.
    pub throttle: Fraction,
    /// Pitch.
    pub pitch: Axis,
    /// Yaw.
    pub yaw: Axis,
    /// Roll.
    pub roll: Axis,
    /// Translation forward.
    pub forward: Axis,
    /// Translation right.
    pub right: Axis,
    /// Translation up.
    pub up: Axis,
    /// Wheel throttle.
    pub wheel_throttle: Axis,
    /// Wheel steering.
    pub wheel_steering: Axis,
    /// Discrete requests, applied after the continuous controls.
    pub requests: Vec<Request>,
}

/// A message from the bridge to the harness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BridgeMessage {
    /// Opens a session.
    Hello {
        /// The bridge's protocol version.
        protocol_version: u32,
    },
    /// The game is held on a tick, and here is every vessel's telemetry for it.
    Tick {
        /// The tick's game time.
        ut: GameTime,
        /// Every vessel the harness controls.
        vessels: Vec<VesselTelemetry>,
    },
    /// The bridge could not carry out a command.
    Refused {
        /// The tick it concerned.
        ut: GameTime,
        /// Why.
        reason: String,
    },
}

/// A message from the harness to the bridge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum HarnessMessage {
    /// Opens a session.
    Hello {
        /// The harness's protocol version.
        protocol_version: u32,
    },
    /// The commands for a tick, after which the bridge advances the game one tick.
    Commands {
        /// The tick the commands answer.
        ut: GameTime,
        /// One entry per vessel commanded.
        commands: Vec<VesselCommand>,
    },
    /// Ends the session and releases the held tick.
    Close,
}

/// Checks that a peer's protocol version matches this crate's.
///
/// # Errors
///
/// Returns [`RecordError::VersionMismatch`] if it differs.
pub fn check_version(theirs: u32) -> Result<(), RecordError> {
    if theirs == PROTOCOL_VERSION {
        Ok(())
    } else {
        Err(RecordError::VersionMismatch {
            ours: PROTOCOL_VERSION,
            theirs,
        })
    }
}

/// Encodes a message as one line of JSON, without the newline.
///
/// # Errors
///
/// Returns [`RecordError::Codec`] if the message cannot be encoded.
pub fn encode<T: Serialize>(message: &T) -> Result<String, RecordError> {
    serde_json::to_string(message).map_err(|e| RecordError::Codec(e.to_string()))
}

/// Decodes one line of JSON into a message, refusing any value that breaks a type's
/// constraints, and any request that breaks its own.
///
/// # Errors
///
/// Returns [`RecordError::Codec`] if the line is not a valid message, or the
/// [`RecordError`] of the first request that fails its check.
pub fn decode_harness_message(line: &str) -> Result<HarnessMessage, RecordError> {
    let m: HarnessMessage =
        serde_json::from_str(line).map_err(|e| RecordError::Codec(e.to_string()))?;
    if let HarnessMessage::Commands { commands, .. } = &m {
        for c in commands {
            for r in &c.requests {
                r.validate()?;
            }
        }
    }
    Ok(m)
}

/// Decodes one line of JSON into a message from the bridge, refusing any value that
/// breaks a type's constraints.
///
/// # Errors
///
/// Returns [`RecordError::Codec`] if the line is not a valid message.
pub fn decode_bridge_message(line: &str) -> Result<BridgeMessage, RecordError> {
    serde_json::from_str(line).map_err(|e| RecordError::Codec(e.to_string()))
}
