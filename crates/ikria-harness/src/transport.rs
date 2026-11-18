//! Wall-clock deadlines for the flight transport.
//!
//! A socket timeout bounds one read or write call only. A peer that sends one byte
//! just inside each timeout can otherwise hold a single frame for hours, and a session
//! of many ticks has no total bound. [`Deadlined`] adds two bounds that fail closed.
//!
//! - **Frame.** Consecutive calls in one direction that carry no newline, which is the
//!   frame terminator, share one frame deadline. It starts at the first call after the
//!   previous newline. A read that returns a newline ends the frame, so a frame read
//!   through a buffered reader waits at most the frame limit from its first blocking
//!   read. A write that sends a newline ends the frame in the same way.
//! - **Session.** Every call refuses once the session deadline has passed.
//!
//! Each call's socket timeout is set to the time left, so no call blocks past either
//! deadline. Expiry is reported as [`io::ErrorKind::TimedOut`] and is permanent, since
//! the session deadline cannot move back. The bounds are wall time on the harness
//! host, measured with a monotonic clock. They are not hard real-time guarantees,
//! because the operating system can still delay the process.
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    time::{Duration, Instant},
};

/// The smallest timeout handed to a stream. A zero timeout means no timeout to
/// `TcpStream`, so the deadline is reported as expired instead.
const MIN_CALL: Duration = Duration::from_millis(1);

/// A stream whose per-call timeouts can be set before each call.
pub trait TimedStream {
    /// Bound the next read call.
    fn set_read_limit(&mut self, limit: Duration) -> io::Result<()>;
    /// Bound the next write call.
    fn set_write_limit(&mut self, limit: Duration) -> io::Result<()>;
}

impl TimedStream for TcpStream {
    fn set_read_limit(&mut self, limit: Duration) -> io::Result<()> {
        self.set_read_timeout(Some(limit.max(MIN_CALL)))
    }
    fn set_write_limit(&mut self, limit: Duration) -> io::Result<()> {
        self.set_write_timeout(Some(limit.max(MIN_CALL)))
    }
}

/// A stream with frame and session deadlines. Use one instance per direction, since
/// each direction keeps its own frame timer, and give both the same session end.
pub struct Deadlined<S, C = fn() -> Instant> {
    inner: S,
    clock: C,
    frame: Duration,
    session_end: Instant,
    read_start: Option<Instant>,
    write_start: Option<Instant>,
}

impl<S> Deadlined<S> {
    /// Wrap a stream with the monotonic system clock.
    pub fn new(inner: S, frame: Duration, session_end: Instant) -> Self {
        Self::with_clock(inner, frame, session_end, Instant::now)
    }
}

impl<S, C: FnMut() -> Instant> Deadlined<S, C> {
    /// Wrap a stream with an explicit clock, which tests use to make time deterministic.
    pub fn with_clock(inner: S, frame: Duration, session_end: Instant, clock: C) -> Self {
        Self {
            inner,
            clock,
            frame,
            session_end,
            read_start: None,
            write_start: None,
        }
    }

    /// Recover the wrapped stream.
    pub fn into_inner(self) -> S {
        self.inner
    }

    /// Whether some bytes of a frame, but not its terminating newline, have been
    /// written. A caller must not append another frame, such as Close, to a partial
    /// one. A write refused before any byte left does not count.
    pub fn mid_write_frame(&self) -> bool {
        self.write_start.is_some()
    }

    /// The time left for a call in a frame that began at `start`, or an error.
    fn budget(&self, start: Instant, now: Instant) -> io::Result<Duration> {
        let end = start
            .checked_add(self.frame)
            .map_or(self.session_end, |end| end.min(self.session_end));
        let left = end.saturating_duration_since(now);
        if left < MIN_CALL {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                if end == self.session_end {
                    "session wall-clock deadline expired"
                } else {
                    "frame wall-clock deadline expired"
                },
            ));
        }
        Ok(left)
    }
}

impl<S: Read + TimedStream, C: FnMut() -> Instant> Read for Deadlined<S, C> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let now = (self.clock)();
        let start = *self.read_start.get_or_insert(now);
        let left = self.budget(start, now)?;
        self.inner.set_read_limit(left)?;
        let n = self.inner.read(buf)?;
        if buf[..n].contains(&b'\n') {
            self.read_start = None;
        }
        Ok(n)
    }
}

impl<S: Write + TimedStream, C: FnMut() -> Instant> Write for Deadlined<S, C> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let now = (self.clock)();
        let start = self.write_start.unwrap_or(now);
        let left = self.budget(start, now)?;
        self.inner.set_write_limit(left)?;
        let n = self.inner.write(buf)?;
        // A frame is open only once one of its bytes has been written, so a call refused
        // before writing anything does not make the stream partial.
        if buf[..n].contains(&b'\n') {
            self.write_start = None;
        } else if n > 0 {
            self.write_start = Some(start);
        }
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
