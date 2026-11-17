//! Run one verification flight and retain an exclusive JSON Lines journal.
use ikria_harness::{
    flight::{HOP, MAX_SOURCE},
    session::{self, Mode},
};
use ikria_records::flight::{HarnessFrame, write_frame};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    net::{SocketAddr, TcpStream},
    time::Duration,
};
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
    socket.set_nodelay(true)?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.set_write_timeout(Some(Duration::from_millis(250)))?;
    let mut reader = BufReader::new(socket.try_clone()?);
    let abort_file = std::env::var_os("IKRIA_ABORT_FILE").map(std::path::PathBuf::from);
    let result = session::run_with_abort(
        &mut reader,
        &mut socket,
        &mut log,
        mode,
        ticks,
        &source,
        || abort_file.as_ref().is_some_and(|path| path.exists()),
    );
    let close = write_frame(&mut socket, &HarnessFrame::Close {});
    let message = serde_json::json!({"event":"session_end","success":result.is_ok(),"error":result.as_ref().err().map(ToString::to_string),"close_sent":close.is_ok()});
    write_frame(&mut log, &message)?;
    log.sync_all()?;
    result
}
