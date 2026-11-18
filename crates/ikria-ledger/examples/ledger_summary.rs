//! Prints a summary of the science ledger of a save, given its path. The file is opened
//! for reading only.
//!
//! ```sh
//! cargo run -p ikria-ledger --example ledger_summary -- path/to/save.sfs
//! ```
//!
//! Values are in base units, the scale of the save's `cap` and `sci` fields, and in points
//! at the save's own science multiplier where the save states one. The exit status is
//! nonzero when the save cannot be read or its subjects cannot be extracted.

use std::path::PathBuf;
use std::process::ExitCode;

use ikria_ledger::{CapCheck, Enumeration, Ledger, Totals, science_scale};
use ikria_save::Save;

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let (Some(path), None) = (args.next(), args.next()) else {
        eprintln!("usage: ledger_summary PATH");
        return ExitCode::from(2);
    };
    let enumeration = match Enumeration::phase0() {
        Ok(e) => e,
        Err(error) => {
            eprintln!("the enumeration is defective: {error}");
            return ExitCode::FAILURE;
        }
    };
    let save = match Save::read(&PathBuf::from(path)) {
        Ok(save) => save,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let ledger = match Ledger::from_save(&enumeration, &save) {
        Ok(ledger) => ledger,
        Err(error) => {
            eprintln!("reading the science subjects: {error}");
            return ExitCode::FAILURE;
        }
    };
    let scale = science_scale(&save);
    match &scale {
        Ok(s) => println!("science multiplier: {}", s.get()),
        Err(error) => println!("science multiplier: not read, {error}"),
    }
    // At a multiplier of one, points equal base units, so the second row is omitted.
    let scale = scale.ok().filter(|s| (s.get() - 1.0).abs() > f64::EPSILON);

    let matched = ledger.totals().in_save;
    let save_subjects = matched + ledger.unmatched().len() + ledger.repeated().len();
    println!(
        "save subjects: {save_subjects}, matched {matched}, unmatched {}, repeated {}",
        ledger.unmatched().len(),
        ledger.repeated().len()
    );
    if save_subjects > 0 {
        #[allow(clippy::cast_precision_loss)]
        let share = matched as f64 / save_subjects as f64 * 100.0;
        println!("matched share: {share:.1} percent");
    }
    println!();
    println!(
        "{:<28} {:>6} {:>6} {:>12} {:>12} {:>12}",
        "group", "count", "saved", "possible", "collected", "remaining"
    );
    row("all", &ledger.totals(), scale);
    for (kind, totals) in ledger.by_kind() {
        row(&format!("{kind:?}"), &totals, scale);
    }
    println!();
    for (body, totals) in ledger.by_body() {
        row(body.name(), &totals, scale);
    }
    println!();
    for (situation, totals) in ledger.by_situation() {
        row(situation.id(), &totals, scale);
    }
    println!();
    for (experiment, totals) in ledger.by_experiment() {
        if totals.in_save > 0 {
            row(experiment, &totals, scale);
        }
    }
    println!("(experiments with no subject in the save are not listed)");

    println!();
    println!(
        "unmatched: {}, holding {:.2}",
        ledger.unmatched().len(),
        ledger.unmatched_collected()
    );
    for record in ledger.unmatched() {
        println!(
            "  {} {:.2} of {:.2}",
            record.id, record.collected, record.cap
        );
    }
    println!("repeated: {}", ledger.repeated().len());
    for record in ledger.repeated() {
        println!(
            "  {} {:.2} of {:.2}",
            record.id, record.collected, record.cap
        );
    }
    let differences: Vec<_> = ledger.cap_differences().collect();
    println!("cap differences: {}", differences.len());
    for entry in differences {
        if let CapCheck::Differs { save, enumerated } = entry.cap_check() {
            println!(
                "  {} save {save:.4}, enumerated {enumerated:.4}",
                entry.subject().id()
            );
        }
    }
    ExitCode::SUCCESS
}

fn row(name: &str, totals: &Totals, scale: Option<ikria_ledger::ScienceScale>) {
    println!(
        "{name:<28} {:>6} {:>6} {:>12.2} {:>12.2} {:>12.2}",
        totals.subjects, totals.in_save, totals.possible, totals.collected, totals.remaining
    );
    if let Some(scale) = scale {
        let (p, c, r) = totals.points(scale);
        println!(
            "{:<28} {:>6} {:>6} {p:>12.2} {c:>12.2} {r:>12.2}",
            "  in points", "", ""
        );
    }
}
