//! A single thread, single vessel scheduler for the T10 skeleton.
//!
//! Identity and increasing game time are checked before the loop is touched. Every
//! error permanently stops this scheduler, since either the loop or the host may have
//! partially advanced. No command is retried. Failure sends no replacement command.
//! A live bridge, hold release, vehicle-specific safe states, campaign rule enforcement,
//! multiple vessels, admission budgets and program swaps remain separate work.

use std::{error::Error, fmt};

use ikria_records::{GameTime, RecordError, VesselId};

use crate::{
    attitude::LoopError,
    control::ControlLoop,
    host::{AppliedCommand, Host},
};

/// Why a scheduler could not complete a tick.
#[derive(Debug)]
pub enum HarnessError<E> {
    /// The host could not read or apply a tick.
    Host(E),
    /// Telemetry addressed another vessel.
    WrongVessel,
    /// Game time repeated or moved backwards.
    NonIncreasingTime,
    /// The control loop failed.
    Loop(LoopError),
    /// The loop returned a command for another vessel.
    WrongCommandVessel,
    /// The command contained an invalid request.
    InvalidCommand(RecordError),
    /// A previous error permanently stopped this scheduler.
    Stopped,
}

impl<E: fmt::Display> fmt::Display for HarnessError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Host(e) => write!(f, "host failed: {e}"),
            Self::WrongVessel => f.write_str("telemetry addresses another vessel"),
            Self::NonIncreasingTime => f.write_str("game time did not increase"),
            Self::Loop(e) => write!(f, "control loop failed: {e}"),
            Self::WrongCommandVessel => f.write_str("command addresses another vessel"),
            Self::InvalidCommand(e) => write!(f, "invalid command: {e}"),
            Self::Stopped => f.write_str("the scheduler stopped after an error"),
        }
    }
}

impl<E: Error + 'static> Error for HarnessError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Host(e) => Some(e),
            Self::Loop(e) => Some(e),
            Self::InvalidCommand(e) => Some(e),
            _ => None,
        }
    }
}

/// Owns one host and one control loop, both confined to the caller's thread.
pub struct Harness<H, C> {
    host: H,
    control: C,
    vessel: VesselId,
    last_time: Option<GameTime>,
    state: State,
}

enum State {
    Running,
    Stopped,
    Finished,
}

impl<H: Host, C: ControlLoop> Harness<H, C> {
    /// Binds a loop to one vessel. Construction does not read or advance the host.
    #[must_use]
    pub fn new(host: H, vessel: VesselId, control: C) -> Self {
        Self {
            host,
            control,
            vessel,
            last_time: None,
            state: State::Running,
        }
    }

    /// Reads, computes and applies one tick, returning its time on success.
    ///
    /// Returns `None` at the end of the source. Later calls also return `None` without
    /// touching the host or the loop. There is no unbounded internal run loop.
    ///
    /// # Errors
    ///
    /// Returns the first host, input, control or output error and permanently stops.
    /// Later calls return [`HarnessError::Stopped`] without touching either component.
    pub fn step(&mut self) -> Result<Option<GameTime>, HarnessError<H::Error>> {
        match self.state {
            State::Stopped => return Err(HarnessError::Stopped),
            State::Finished => return Ok(None),
            State::Running => {}
        }
        // Set before any fallible work. Only a completed cycle or clean exhaustion
        // clears the latch, including when command application has uncertain effects.
        self.state = State::Stopped;
        let Some(tick) = self.host.next_tick().map_err(HarnessError::Host)? else {
            self.state = State::Finished;
            return Ok(None);
        };
        if tick.vessel.id != self.vessel {
            return Err(HarnessError::WrongVessel);
        }
        if self.last_time.is_some_and(|last| tick.ut <= last) {
            return Err(HarnessError::NonIncreasingTime);
        }
        let command = self
            .control
            .step(&tick.vessel)
            .map_err(HarnessError::Loop)?;
        if command.id != self.vessel {
            return Err(HarnessError::WrongCommandVessel);
        }
        for request in &command.requests {
            request.validate().map_err(HarnessError::InvalidCommand)?;
        }
        self.host
            .apply_command(AppliedCommand {
                ut: tick.ut,
                command,
            })
            .map_err(HarnessError::Host)?;
        self.last_time = Some(tick.ut);
        self.state = State::Running;
        Ok(Some(tick.ut))
    }

    /// Inspects the host without allowing its tick state to be changed.
    #[must_use]
    pub fn host(&self) -> &H {
        &self.host
    }

    /// Consumes the scheduler and returns the host, including any held tick on failure.
    #[must_use]
    pub fn into_host(self) -> H {
        self.host
    }
}
