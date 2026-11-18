//! A single vessel host contract and a deterministic, in-memory scripted host.
//!
//! Reading holds a tick. Only a successful command application permits reading the
//! next tick. This models lockstep ordering without sockets, wall time or the game.

use std::{collections::VecDeque, error::Error, fmt};

use ikria_records::{GameTime, RecordError, VesselCommand, VesselId, VesselTelemetry};

/// One vessel's telemetry at a held tick.
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    /// The tick's game time.
    pub ut: GameTime,
    /// The controlled vessel's state.
    pub vessel: VesselTelemetry,
}

/// An owned record of commands successfully applied to one tick.
#[derive(Debug, Clone, PartialEq)]
pub struct AppliedCommand {
    /// The tick's game time.
    pub ut: GameTime,
    /// The command applied to the vessel.
    pub command: VesselCommand,
}

/// The boundary between a scheduler and a source of held telemetry.
///
/// Implementations must not advance a tick until its command succeeds. A failure can
/// leave application uncertain, so the scheduler never retries it automatically. A
/// future bridge host must also arrange release of a hold when its session ends.
pub trait Host {
    /// A host-specific failure.
    type Error: Error + 'static;

    /// Holds and reads the next tick, or returns `None` at the end of the source.
    ///
    /// # Errors
    ///
    /// Fails if the previous tick is still held or the source cannot be read.
    fn next_tick(&mut self) -> Result<Option<Tick>, Self::Error>;

    /// Applies one owned command to the held tick, then permits the next read.
    ///
    /// # Errors
    ///
    /// Fails if there is no matching held tick or application fails.
    fn apply_command(&mut self, command: AppliedCommand) -> Result<(), Self::Error>;
}

/// A scripted host refused an operation. The held tick remains unchanged.
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptError {
    /// Another read was attempted before applying the previous tick's command.
    TickHeld,
    /// A command arrived without a held tick.
    NoHeldTick,
    /// A command's time does not match the held tick.
    WrongTime,
    /// A command addresses another vessel.
    WrongVessel,
    /// A command contains an invalid request.
    InvalidCommand(RecordError),
}

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TickHeld => f.write_str("the previous tick is still held"),
            Self::NoHeldTick => f.write_str("there is no held tick"),
            Self::WrongTime => f.write_str("the command does not answer the held tick"),
            Self::WrongVessel => f.write_str("the command addresses another vessel"),
            Self::InvalidCommand(e) => write!(f, "invalid command: {e}"),
        }
    }
}

impl Error for ScriptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidCommand(e) => Some(e),
            _ => None,
        }
    }
}

/// Replays a finite telemetry script and retains every successfully applied command.
///
/// Scripts are test inputs, not a physics model. Identity and chronological checks
/// belong to the scheduler, so malformed scripts can exercise those refusals too.
/// Storage grows with the script and the command log, and is not a flight resource bound.
#[derive(Debug)]
pub struct ScriptedHost {
    ticks: VecDeque<Tick>,
    held: Option<(GameTime, VesselId)>,
    applied: Vec<AppliedCommand>,
}

impl ScriptedHost {
    /// Takes ownership of a finite script. An empty script is valid.
    #[must_use]
    pub fn new(ticks: Vec<Tick>) -> Self {
        Self {
            ticks: ticks.into(),
            held: None,
            applied: Vec::new(),
        }
    }

    /// The commands successfully applied so far, in script order.
    #[must_use]
    pub fn applied(&self) -> &[AppliedCommand] {
        &self.applied
    }
}

impl Host for ScriptedHost {
    type Error = ScriptError;

    fn next_tick(&mut self) -> Result<Option<Tick>, Self::Error> {
        if self.held.is_some() {
            return Err(ScriptError::TickHeld);
        }
        let tick = self.ticks.pop_front();
        self.held = tick.as_ref().map(|t| (t.ut, t.vessel.id));
        Ok(tick)
    }

    fn apply_command(&mut self, applied: AppliedCommand) -> Result<(), Self::Error> {
        let (ut, id) = self.held.ok_or(ScriptError::NoHeldTick)?;
        if applied.ut != ut {
            return Err(ScriptError::WrongTime);
        }
        if applied.command.id != id {
            return Err(ScriptError::WrongVessel);
        }
        for request in &applied.command.requests {
            request.validate().map_err(ScriptError::InvalidCommand)?;
        }
        self.applied.push(applied);
        self.held = None;
        Ok(())
    }
}
