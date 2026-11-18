//! The science ledger, roadmap task T9.
//!
//! [`enumeration`] lists every science subject that the Phase 0 count of
//! `record/verification/VR-0005.md` counts, one by one, with the identifier the game
//! writes in a save and its value. [`catalog`] holds the rules it applies, ported from
//! `tools/phase0/science_subjects.py`. [`ledger`] joins the enumeration with a save's
//! science subjects, read through `ikria-save`, and gives each subject's possible,
//! collected and remaining science, with totals by body, experiment and situation.
//!
//! # Trust
//!
//! The save is untrusted input, read by `ikria-save` under its limits. The ledger only
//! compares identifiers and sums numbers that the reader has already checked to be finite
//! and not negative. It never writes, and no input is known to make it panic.
//!
//! # Agreement with Phase 0
//!
//! The tests in `tests/phase0.rs` hold the ported tables in agreement with the script and
//! the tracked inventory, and recompute the record's totals from the enumeration. The
//! subject counts and the Hard values all agree exactly, after the correction of the
//! script's surface feature split on 2026-11-18. The crate's `README.md` gives the
//! details, and lists the assumptions that no save has yet verified.

#![forbid(unsafe_code)]

pub mod catalog;
pub mod enumeration;
pub mod ledger;

pub use catalog::{Body, Experiment, LocationBiome, Multiplier, Provenance, Situation};
pub use enumeration::{
    Biome, CountTotals, Enumeration, EnumerationError, Hundredths, Source, Subject, SubjectKind,
};
pub use ledger::{
    CapCheck, Entry, Ledger, MultiplierCheck, SaveAccount, SaveRecord, ScaleError, ScienceScale,
    Totals, agrees, science_scale,
};
