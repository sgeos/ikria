//! Run one verification flight and retain an exclusive JSON Lines journal.
use ikria_harness::{
    flight::{HOP, MAX_SOURCE},
    session::{self, Mode},
    transport::Deadlined,
};
use ikria_records::flight::{HarnessFrame, write_frame};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    net::{SocketAddr, TcpStream},
    time::{Duration, Instant},
};

/// Wall-clock limit for one bridge frame. It matches the former per-read timeout and
/// exceeds the bridge's 700 millisecond hold of a tick.
const READ_FRAME: Duration = Duration::from_secs(2);
/// Wall-clock limit for one harness frame, the former per-write timeout.
const WRITE_FRAME: Duration = Duration::from_millis(250);
/// Wall-clock limit for the whole session from connection. It is below the 840 second
/// limit that `tools/phase0/run_flight.py` places on this process, so that the harness
/// ends, closes and synchronizes its journal before the supervisor would kill it. A
/// session that needs longer fails closed as a transport fault.
const SESSION: Duration = Duration::from_secs(720);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !(args.len() == 4 || args.len() == 5) {
        return Err(
            "usage: fly LOOPBACK_ADDRESS LOG_PATH monitor|hop|pad-abort MAX_TICKS [PROGRAM_BODY]"
                .into(),
        );
    }
    let address: SocketAddr = args[0].parse()?;
    if !address.ip().is_loopback() {
        return Err("only loopback transport is supported".into());
    }
    let mode = match args[2].as_str() {
        "monitor" => Mode::Monitor,
        "hop" => Mode::Hop,
        "pad-abort" => Mode::PadAbort,
        _ => return Err("unknown mode".into()),
    };
    let ticks = args[3].parse()?;
    let source = if args.len() == 5 {
        let mut source = String::new();
        std::fs::File::open(&args[4])?
            .take((MAX_SOURCE + 1) as u64)
            .read_to_string(&mut source)?;
        source
    } else {
        HOP.into()
    };
    let mut log = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[1])?;
    let mut socket = TcpStream::connect_timeout(&address, Duration::from_secs(2))?;
    let session_end = Instant::now() + SESSION;
    socket.set_nodelay(true)?;
    socket.set_read_timeout(Some(READ_FRAME))?;
    socket.set_write_timeout(Some(WRITE_FRAME))?;
    let mut reader = BufReader::new(Deadlined::new(socket.try_clone()?, READ_FRAME, session_end));
    let mut writer = Deadlined::new(socket.try_clone()?, WRITE_FRAME, session_end);
    let abort_file = std::env::var_os("IKRIA_ABORT_FILE").map(std::path::PathBuf::from);
    let result = session::run_with_abort(
        &mut reader,
        &mut writer,
        &mut log,
        mode,
        ticks,
        &source,
        || session::abort_file_requested(abort_file.as_deref()),
    );
    // Close is sent on the raw socket, outside the session deadline, but never after
    // a partial frame, which it would corrupt. Disconnection also releases the tick.
    let close = if writer.mid_write_frame() {
        Err("a partial frame was written, so Close was withheld".to_string())
    } else {
        write_frame(&mut socket, &HarnessFrame::Close {}).map_err(|e| e.to_string())
    };
    let message = serde_json::json!({"event":"session_end","success":result.is_ok(),"error":result.as_ref().err().map(ToString::to_string),"close_sent":close.is_ok(),"close_error":close.err()});
    write_frame(&mut log, &message)?;
    log.sync_all()?;
    result
}
