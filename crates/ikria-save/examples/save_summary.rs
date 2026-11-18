//! Prints a summary of a save, given its path. The file is opened for reading only.
//!
//! ```sh
//! cargo run -p ikria-save --example save_summary -- path/to/save.sfs
//! ```
//!
//! Each extractor's failure is printed in place of its section, so that one malformed
//! section does not hide the others. The exit status is nonzero when the file cannot
//! be read or parsed, or when any section failed.

use std::path::PathBuf;
use std::process::ExitCode;

use ikria_save::{ExtractError, Save};

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let (Some(path), None) = (args.next(), args.next()) else {
        eprintln!("usage: save_summary PATH");
        return ExitCode::from(2);
    };
    let path = PathBuf::from(path);
    let save = match Save::read(&path) {
        Ok(save) => save,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let results = [
        section(
            "agency",
            save.agency().map(|a| {
                vec![format!(
                    "funds {:.2}, science {:.2}, reputation {:.2}",
                    a.funds.get(),
                    a.science.get(),
                    a.reputation.get()
                )]
            }),
        ),
        section(
            "facilities",
            save.facilities().map(|list| {
                list.iter()
                    .map(|f| format!("{:?} level {}", f.facility, f.level.displayed()))
                    .collect()
            }),
        ),
        section(
            "technologies",
            save.technologies().map(|list| {
                let parts: usize = list.iter().map(|t| t.parts.len()).sum();
                vec![format!("{} nodes, {parts} part entries", list.len())]
            }),
        ),
        section(
            "science subjects",
            save.science_subjects().map(|list| {
                list.iter()
                    .map(|s| format!("{} {:.2} of {:.2}", s.id, s.science.get(), s.cap.get()))
                    .collect()
            }),
        ),
        section(
            "roster",
            save.roster().map(|list| {
                list.iter()
                    .map(|k| {
                        format!(
                            "{} {:?} {:?} {:?}",
                            k.name, k.kerbal_type, k.kerbal_trait, k.status
                        )
                    })
                    .collect()
            }),
        ),
        section(
            "vessels",
            save.vessels().map(|list| {
                list.iter()
                    .map(|v| {
                        format!(
                            "{} {:?} {:?} at {}, {} parts, crew {:?}",
                            v.name,
                            v.vessel_type,
                            v.situation,
                            v.body
                                .stock_name()
                                .unwrap_or("a body past the stock system"),
                            v.part_count,
                            v.crew
                        )
                    })
                    .collect()
            }),
        ),
        section(
            "contracts",
            save.contracts().map(|list| {
                list.iter()
                    .map(|c| format!("{} {:?} {:?}", c.contract_type, c.state, c.list))
                    .collect()
            }),
        ),
    ];
    if results.iter().all(|ok| *ok) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn section(title: &str, lines: Result<Vec<String>, ExtractError>) -> bool {
    match lines {
        Ok(lines) => {
            println!("{title}: {}", lines.len());
            for line in lines {
                println!("  {line}");
            }
            true
        }
        Err(error) => {
            println!("{title}: error, {error}");
            false
        }
    }
}
