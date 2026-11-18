//! Wall-clock frame and session deadlines, with a simulated clock and stream.
use ikria_harness::transport::{Deadlined, TimedStream};
use ikria_records::flight::{self, HarnessFrame};
use std::{
    cell::Cell,
    collections::VecDeque,
    io::{self, BufReader, Read, Write},
    rc::Rc,
    time::{Duration, Instant},
};

/// A stream whose every call takes `step` of simulated time. Reads return one chunk
/// per call. Writes accept at most `accept` bytes per call. A call whose limit is
/// shorter than `step` times out when its limit expires, as a socket would.
struct Simulated {
    now: Rc<Cell<Instant>>,
    step: Duration,
    chunks: VecDeque<Vec<u8>>,
    accept: usize,
    written: Vec<u8>,
    /// Each limit set, with the simulated time at which it was set.
    limits: Vec<(Instant, Duration)>,
}
impl Simulated {
    fn new(now: &Rc<Cell<Instant>>, step: Duration, chunks: Vec<&[u8]>) -> Self {
        Self {
            now: Rc::clone(now),
            step,
            chunks: chunks.into_iter().map(<[u8]>::to_vec).collect(),
            accept: usize::MAX,
            written: Vec::new(),
            limits: Vec::new(),
        }
    }
    /// Spend one call's time, or time out at the call's limit.
    fn advance(&self) -> io::Result<()> {
        let limit = self.limits.last().map_or(self.step, |&(_, limit)| limit);
        if limit < self.step {
            self.now.set(self.now.get() + limit);
            return Err(io::Error::new(io::ErrorKind::TimedOut, "simulated timeout"));
        }
        self.now.set(self.now.get() + self.step);
        Ok(())
    }
}
impl Read for Simulated {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.advance()?;
        let Some(chunk) = self.chunks.pop_front() else {
            return Ok(0);
        };
        assert!(chunk.len() <= buf.len());
        buf[..chunk.len()].copy_from_slice(&chunk);
        Ok(chunk.len())
    }
}
impl Write for Simulated {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.advance()?;
        let n = buf.len().min(self.accept);
        self.written.extend_from_slice(&buf[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl TimedStream for Simulated {
    fn set_read_limit(&mut self, limit: Duration) -> io::Result<()> {
        self.limits.push((self.now.get(), limit));
        Ok(())
    }
    fn set_write_limit(&mut self, limit: Duration) -> io::Result<()> {
        self.limits.push((self.now.get(), limit));
        Ok(())
    }
}
fn clock(now: &Rc<Cell<Instant>>) -> impl FnMut() -> Instant + use<> {
    let now = Rc::clone(now);
    move || now.get()
}
const FAR: Duration = Duration::from_secs(3600);

#[test]
fn slow_drip_frame_is_cut_at_the_frame_deadline() {
    let start = Instant::now();
    let now = Rc::new(Cell::new(start));
    let frame = Duration::from_secs(1);
    let step = Duration::from_millis(300);
    let stream = Simulated::new(&now, step, vec![b"x"; 1000]);
    let mut reader = BufReader::new(Deadlined::with_clock(
        stream,
        frame,
        start + FAR,
        clock(&now),
    ));
    assert!(flight::read_frame::<_, serde_json::Value>(&mut reader).is_err());
    // The last call timed out at the frame deadline, not one step later.
    assert_eq!(now.get(), start + frame);
    // The frame never ended, so every later attempt is refused without a call.
    let error = flight::read_frame::<_, serde_json::Value>(&mut reader).unwrap_err();
    assert!(error.to_string().contains("frame wall-clock deadline"));
    assert_eq!(now.get(), start + frame);
    let stream = reader.into_inner().into_inner();
    // Without the deadline the frame would take all 1000 bytes. With it, only the
    // calls that complete inside the frame limit deliver a byte.
    let reads = 1000 - stream.chunks.len();
    assert_eq!(reads as u128, frame.as_millis() / step.as_millis());
    for (at, limit) in stream.limits {
        assert!(at + limit <= start + frame);
    }
}

#[test]
fn complete_frames_each_get_a_full_frame_limit() {
    let start = Instant::now();
    let now = Rc::new(Cell::new(start));
    let frame = Duration::from_secs(1);
    let step = Duration::from_millis(600);
    let stream = Simulated::new(&now, step, vec![b"{}\n"; 10]);
    let mut reader = BufReader::new(Deadlined::with_clock(
        stream,
        frame,
        start + FAR,
        clock(&now),
    ));
    for _ in 0..10 {
        flight::read_frame::<_, serde_json::Value>(&mut reader).unwrap();
    }
    // The ten frames together took longer than one frame limit.
    assert_eq!(now.get() - start, step * 10);
    assert!(step * 10 > frame);
}

#[test]
fn session_deadline_refuses_every_later_call() {
    let start = Instant::now();
    let now = Rc::new(Cell::new(start));
    let session = Duration::from_secs(2);
    let step = Duration::from_millis(600);
    let stream = Simulated::new(&now, step, vec![b"{}\n"; 10]);
    let mut reader = BufReader::new(Deadlined::with_clock(
        stream,
        Duration::from_secs(1),
        start + session,
        clock(&now),
    ));
    let mut frames = 0;
    let mut errors = Vec::new();
    for _ in 0..10 {
        match flight::read_frame::<_, serde_json::Value>(&mut reader) {
            Ok(_) => {
                assert!(errors.is_empty(), "a call succeeded after a deadline");
                frames += 1;
            }
            Err(error) => errors.push(error.to_string()),
        }
    }
    assert_eq!(frames, session.as_millis() / step.as_millis());
    assert!(
        errors
            .last()
            .unwrap()
            .contains("session wall-clock deadline")
    );
    assert_eq!(now.get(), start + session);
    for (at, limit) in reader.into_inner().into_inner().limits {
        assert!(at + limit <= start + session);
    }
}

#[test]
fn partial_write_is_cut_and_reported_as_mid_frame() {
    let start = Instant::now();
    let now = Rc::new(Cell::new(start));
    let frame = Duration::from_millis(250);
    let step = Duration::from_millis(100);
    // A whole frame written in one call leaves no frame open.
    let stream = Simulated::new(&now, step, Vec::new());
    let mut whole = Deadlined::with_clock(stream, frame, start + FAR, clock(&now));
    flight::write_frame(&mut whole, &HarnessFrame::Close {}).unwrap();
    assert!(!whole.mid_write_frame());
    // A peer that accepts one byte per call cannot hold the frame open past the limit.
    let mut stream = Simulated::new(&now, step, Vec::new());
    stream.accept = 1;
    let begun = now.get();
    let mut drip = Deadlined::with_clock(stream, frame, start + FAR, clock(&now));
    assert!(flight::write_frame(&mut drip, &HarnessFrame::Close {}).is_err());
    assert!(drip.mid_write_frame());
    assert_eq!(now.get() - begun, frame);
    let written = drip.into_inner().written;
    assert_eq!(written.len() as u128, frame.as_millis() / step.as_millis());
}
