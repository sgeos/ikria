//! Deterministically replay a completed flight journal without the bridge or game.
use ikria_harness::{
    flight::{ABORT, DIALOGUE, HOP, MAX_SOURCE},
    session::{self, Mode},
};
use ikria_records::flight::{self, BridgeFrame, HarnessFrame};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Cursor, Read},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !(1..=3).contains(&args.len()) {
        return Err("usage: replay JOURNAL.jsonl [PROGRAM_BODY [ABORT_BODY]]".into());
    }
    let source = if args.len() >= 2 {
        let mut text = String::new();
        File::open(&args[1])?
            .take((MAX_SOURCE + 1) as u64)
            .read_to_string(&mut text)?;
        if text.len() > MAX_SOURCE {
            return Err("program body exceeds size limit".into());
        }
        text
    } else {
        HOP.into()
    };
    // Offline replay may allocate up to 64 MiB of input, separate from flight budgets.
    let file = File::open(&args[0])?;
    if file.metadata()?.len() > 64 * 1024 * 1024 {
        return Err("journal exceeds 64 MiB".into());
    }
    let mut input = BufReader::new(file.take(64 * 1024 * 1024));
    let header: Value = flight::read_frame(&mut input)?;
    if header["event"] != "session"
        || header["program_sha256"]
            != format!("{:x}", Sha256::digest(format!("{DIALOGUE}\n{source}")))
    {
        return Err("journal is not for the bundled hop program".into());
    }
    let hash = |body: &str| format!("{:x}", Sha256::digest(format!("{DIALOGUE}\n{body}")));
    let abort_source = if args.len() == 3 {
        let mut text = String::new();
        File::open(&args[2])?
            .take((MAX_SOURCE + 1) as u64)
            .read_to_string(&mut text)?;
        if text.len() > MAX_SOURCE {
            return Err("abort body exceeds size limit".into());
        }
        text
    } else {
        // Journals before the distinct abort program used the hop source for both.
        // The recorded hash selects among the known bodies and is checked below.
        [source.as_str(), ABORT, HOP]
            .into_iter()
            .find(|body| header["abort_sha256"] == hash(body))
            .unwrap_or(ABORT)
            .to_string()
    };
    if header["abort_sha256"] != hash(&abort_source) {
        return Err("abort program hash mismatch".into());
    }
    let mode = match header["mode"].as_str() {
        Some("Monitor") => Mode::Monitor,
        Some("Hop") => Mode::Hop,
        Some("PadAbort") => Mode::PadAbort,
        _ => return Err("unknown mode".into()),
    };
    let hello: BridgeFrame = serde_json::from_value(header["hello"].clone())?;
    let mut wire = Vec::new();
    flight::write_frame(&mut wire, &hello)?;
    let mut expected = Vec::new();
    let mut ticks = 0;
    let mut aborted_at = None;
    let mut complete = false;
    let mut ended = false;
    for _ in 0..100000 {
        let value: Value = flight::read_frame(&mut input)?;
        if value["event"] == "session_end" {
            if value["success"] != true {
                return Err("incomplete or failed journal cannot certify a replay".into());
            }
            ended = true;
            break;
        }
        if value["event"] == "abort_requested" {
            aborted_at = value["seq"].as_u64();
        }
        if value["event"] == "complete" {
            complete = true;
        }
        if value["event"] == "command_intent" {
            expected.push(serde_json::from_value::<HarnessFrame>(
                value["request"].clone(),
            )?);
        }
        if value.get("type").is_some() {
            let frame: BridgeFrame = serde_json::from_value(value)?;
            if matches!(frame, BridgeFrame::Sample { .. }) {
                ticks += 1;
            }
            flight::write_frame(&mut wire, &frame)?;
        }
    }
    if !ended || (mode != Mode::Monitor && !complete) {
        return Err("journal has no successful terminal record".into());
    }
    let mut remainder = String::new();
    input.read_to_string(&mut remainder)?;
    if !remainder.trim().is_empty() {
        return Err("unexpected data after terminal record".into());
    }
    let mut commands = Vec::new();
    let mut counter = 0;
    session::run_with_sources(
        &mut Cursor::new(wire),
        &mut commands,
        &mut std::io::sink(),
        mode,
        ticks,
        session::ProgramSources {
            normal: &source,
            abort: &abort_source,
        },
        || {
            let requested = aborted_at == Some(counter);
            counter += 1;
            requested
        },
    )?;
    let mut output = Cursor::new(commands);
    let _: HarnessFrame = flight::read_frame(&mut output)?;
    for command in &expected {
        let replayed: HarnessFrame = flight::read_frame(&mut output)?;
        if &replayed != command {
            return Err("replayed command differs from recorded intent".into());
        }
    }
    if output.position() != output.get_ref().len() as u64 {
        return Err("replay produced extra commands".into());
    }
    println!(
        "replayed {ticks} samples and {} identical commands",
        expected.len()
    );
    Ok(())
}
