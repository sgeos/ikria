//! The ledger, which joins the enumeration with a save's science subjects.
//!
//! # Units
//!
//! A save's `cap` and `sci` are taken to be in base units, before the difficulty's
//! science multiplier. This is an *inference*. Two saves at multipliers of 1 and 0.6 hold
//! the same `cap` for the same recovery subjects, and the older save's
//! `mysteryGoo@KerbinFlyingLow` holds a `cap` of 9.1, which is the cap of 13 times
//! Kerbin's flying low multiplier of 0.7 with nothing else applied. The ledger therefore
//! works in base units, and [`Totals::points`] converts to the points the agency
//! receives at a given multiplier.
//!
//! # Matching
//!
//! A save subject matches the enumeration only when its identifier is equal, byte for
//! byte. Nothing is normalized, so a difference of case is a mismatch. A placeholder
//! subject of an unnamed location never matches, even when a save holds its identifier,
//! since that identifier is not the game's. A save subject that matches nothing is kept
//! as unmatched with its science, and a save subject whose identifier repeats an earlier
//! one is kept as repeated. The first of a repeated identifier is the one joined. No save
//! subject is dropped, and [`Ledger::save_account`] sums the science of all three kinds
//! of record.

use std::collections::BTreeMap;
use std::collections::HashMap;

use core::fmt;

use ikria_save::{ExtractError, Save, ScienceSubject};

use crate::catalog::{Body, Situation};
use crate::enumeration::{Enumeration, Subject, SubjectKind};

/// How far a save's cap may sit from the enumeration's value and still agree. Saves
/// store single precision numbers, whose relative error is about 6e-8, so this is wide
/// enough for rounding and far too narrow to hide a different multiplier.
pub const CAP_RELATIVE_TOLERANCE: f64 = 1e-5;

/// The absolute floor of the cap tolerance, for caps near zero.
pub const CAP_ABSOLUTE_TOLERANCE: f64 = 1e-4;

/// The difficulty's science multiplier, finite and above zero.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ScienceScale(f64);

impl ScienceScale {
    /// The Hard difficulty's multiplier.
    pub const HARD: Self = Self(0.6);

    /// Wraps a multiplier that is finite and above zero.
    #[must_use]
    pub fn new(value: f64) -> Option<Self> {
        (value.is_finite() && value > 0.0).then_some(Self(value))
    }

    /// The multiplier.
    #[must_use]
    pub fn get(self) -> f64 {
        self.0
    }
}

/// Why a save's science multiplier could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScaleError {
    /// The save has no `PARAMETERS` node with a `CAREER` node holding
    /// `ScienceGainMultiplier`. The path that was missing is named.
    Missing(&'static str),
    /// The value is not a number above zero.
    Invalid(String),
}

impl fmt::Display for ScaleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(path) => write!(f, "the save has no `{path}`"),
            Self::Invalid(value) => write!(
                f,
                "`ScienceGainMultiplier` is `{value}`, expected a number above zero"
            ),
        }
    }
}

impl std::error::Error for ScaleError {}

/// Reads the difficulty's science multiplier from `GAME`, `PARAMETERS`, `CAREER`,
/// `ScienceGainMultiplier`, which is 0.6 on Hard.
///
/// # Errors
///
/// A [`ScaleError`] when the field is absent or not a number above zero.
pub fn science_scale(save: &Save) -> Result<ScienceScale, ScaleError> {
    let parameters = save
        .game()
        .child("PARAMETERS")
        .ok_or(ScaleError::Missing("PARAMETERS"))?;
    let career = parameters
        .child("CAREER")
        .ok_or(ScaleError::Missing("PARAMETERS/CAREER"))?;
    let value = career
        .value("ScienceGainMultiplier")
        .ok_or(ScaleError::Missing(
            "PARAMETERS/CAREER/ScienceGainMultiplier",
        ))?;
    value
        .value()
        .trim()
        .parse::<f64>()
        .ok()
        .and_then(ScienceScale::new)
        .ok_or_else(|| ScaleError::Invalid(value.value().chars().take(64).collect()))
}

/// The part of a save subject that the ledger keeps.
#[derive(Debug, Clone, PartialEq)]
pub struct SaveRecord {
    /// The identifier.
    pub id: String,
    /// Science collected, in base units, from `sci`.
    pub collected: f64,
    /// The save's own cap, in base units, from `cap`.
    pub cap: f64,
    /// The save's own multiplier of the body and situation, from `sbv`.
    pub multiplier: f64,
}

impl SaveRecord {
    fn from_subject(subject: &ScienceSubject) -> Self {
        Self {
            id: subject.id.clone(),
            collected: subject.science.get(),
            cap: subject.cap.get(),
            multiplier: subject.subject_value.get(),
        }
    }
}

/// Whether a value read from a save agrees with the enumeration's, within the tolerance
/// that the cap and multiplier checks and the remaining science share.
#[must_use]
pub fn agrees(save: f64, enumerated: f64) -> bool {
    let tolerance = (enumerated.abs() * CAP_RELATIVE_TOLERANCE).max(CAP_ABSOLUTE_TOLERANCE);
    (save - enumerated).abs() <= tolerance
}

/// How a save's cap compares with the enumeration's value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CapCheck {
    /// The save holds no such subject, so there is nothing to compare.
    NotInSave,
    /// The two agree within the tolerance.
    Agrees,
    /// The two differ. Neither is chosen silently. The ledger's possible science is the
    /// enumeration's value, and this variant reports the difference.
    Differs {
        /// The save's cap.
        save: f64,
        /// The enumeration's base value.
        enumerated: f64,
    },
}

impl CapCheck {
    fn compare(save: f64, enumerated: f64) -> Self {
        if agrees(save, enumerated) {
            Self::Agrees
        } else {
            Self::Differs { save, enumerated }
        }
    }
}

/// How a save's own multiplier, its `sbv`, compares with the enumeration's multiplier.
///
/// This is reported apart from the cap, because it names the cause. A wrong body or
/// situation multiplier in the tables, such as the unverified multiplier of one that
/// surface features are given, shows here the first time a save holds such a subject.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MultiplierCheck {
    /// The save holds no such subject, so there is nothing to compare.
    NotInSave,
    /// The two agree within the tolerance.
    Agrees,
    /// The two differ.
    Differs {
        /// The save's `sbv`.
        save: f64,
        /// The enumeration's multiplier.
        enumerated: f64,
    },
}

impl MultiplierCheck {
    fn compare(save: f64, enumerated: f64) -> Self {
        if agrees(save, enumerated) {
            Self::Agrees
        } else {
            Self::Differs { save, enumerated }
        }
    }
}

/// One subject of the enumeration and what the save holds for it.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry<'e> {
    subject: &'e Subject,
    save: Option<SaveRecord>,
}

impl<'e> Entry<'e> {
    /// The subject.
    #[must_use]
    pub fn subject(&self) -> &'e Subject {
        self.subject
    }

    /// The save's record of it, if any.
    #[must_use]
    pub fn save(&self) -> Option<&SaveRecord> {
        self.save.as_ref()
    }

    /// The possible science in base units, the enumeration's value.
    #[must_use]
    pub fn possible(&self) -> f64 {
        self.subject.base_value().as_points()
    }

    /// The science collected in base units, zero when the save holds no record.
    #[must_use]
    pub fn collected(&self) -> f64 {
        self.save.as_ref().map_or(0.0, |s| s.collected)
    }

    /// The science remaining in base units, the possible less the collected, and never
    /// below zero. It is exactly zero when the collected science agrees with the possible
    /// within the cap check's tolerance, since the game stores single precision numbers
    /// and a fully collected subject otherwise leaves a residue of a few millionths.
    /// Where the save holds more than the enumeration allows, see [`Entry::cap_check`] and
    /// [`Entry::excess`].
    #[must_use]
    pub fn remaining(&self) -> f64 {
        let (possible, collected) = (self.possible(), self.collected());
        if agrees(collected, possible) {
            0.0
        } else {
            (possible - collected).max(0.0)
        }
    }

    /// The science collected beyond the enumeration's value, which is zero unless the
    /// save's cap is higher than the enumeration's. Like [`Entry::remaining`], it is
    /// exactly zero within the tolerance.
    #[must_use]
    pub fn excess(&self) -> f64 {
        let (possible, collected) = (self.possible(), self.collected());
        if agrees(collected, possible) {
            0.0
        } else {
            (collected - possible).max(0.0)
        }
    }

    /// How the save's own multiplier compares with the enumeration's.
    #[must_use]
    pub fn multiplier_check(&self) -> MultiplierCheck {
        match &self.save {
            None => MultiplierCheck::NotInSave,
            Some(record) => {
                MultiplierCheck::compare(record.multiplier, self.subject.multiplier().get())
            }
        }
    }

    /// How the save's cap compares with the enumeration's value.
    #[must_use]
    pub fn cap_check(&self) -> CapCheck {
        match &self.save {
            None => CapCheck::NotInSave,
            Some(record) => CapCheck::compare(record.cap, self.possible()),
        }
    }
}

/// Totals of a group of entries, in base units.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Totals {
    /// The number of subjects of the enumeration in the group.
    pub subjects: usize,
    /// How many of them the save holds.
    pub in_save: usize,
    /// How many of them have a cap that differs from the save's.
    pub cap_differences: usize,
    /// How many of them have a multiplier that differs from the save's `sbv`.
    pub multiplier_differences: usize,
    /// The possible science.
    pub possible: f64,
    /// The science collected.
    pub collected: f64,
    /// The science remaining, summed per subject, so collection beyond one subject's
    /// value does not offset another's remainder.
    pub remaining: f64,
}

impl Totals {
    fn add(&mut self, entry: &Entry<'_>) {
        self.subjects = self.subjects.saturating_add(1);
        if entry.save.is_some() {
            self.in_save = self.in_save.saturating_add(1);
        }
        if matches!(entry.cap_check(), CapCheck::Differs { .. }) {
            self.cap_differences = self.cap_differences.saturating_add(1);
        }
        if matches!(entry.multiplier_check(), MultiplierCheck::Differs { .. }) {
            self.multiplier_differences = self.multiplier_differences.saturating_add(1);
        }
        self.possible += entry.possible();
        self.collected += entry.collected();
        self.remaining += entry.remaining();
    }

    /// The possible, collected and remaining science in points at a multiplier.
    #[must_use]
    pub fn points(&self, scale: ScienceScale) -> (f64, f64, f64) {
        let s = scale.get();
        (self.possible * s, self.collected * s, self.remaining * s)
    }
}

/// The science ledger of one save.
#[derive(Debug, Clone, PartialEq)]
pub struct Ledger<'e> {
    entries: Vec<Entry<'e>>,
    unmatched: Vec<SaveRecord>,
    repeated: Vec<SaveRecord>,
}

impl<'e> Ledger<'e> {
    /// Joins the enumeration with a save's subjects, in the save's order.
    #[must_use]
    pub fn build(enumeration: &'e Enumeration, subjects: &[ScienceSubject]) -> Self {
        let mut found: HashMap<&str, SaveRecord> = HashMap::with_capacity(subjects.len());
        let mut unmatched = Vec::new();
        let mut repeated = Vec::new();
        for subject in subjects {
            let record = SaveRecord::from_subject(subject);
            let known = enumeration
                .get(&subject.id)
                .filter(|known| !known.is_placeholder());
            if found.contains_key(subject.id.as_str()) {
                repeated.push(record);
            } else if let Some(known) = known {
                found.insert(known.id(), record);
            } else if unmatched.iter().any(|u: &SaveRecord| u.id == subject.id) {
                repeated.push(record);
            } else {
                unmatched.push(record);
            }
        }
        let entries = enumeration
            .subjects()
            .iter()
            .map(|subject| Entry {
                subject,
                save: found.remove(subject.id()),
            })
            .collect();
        Self {
            entries,
            unmatched,
            repeated,
        }
    }

    /// Reads a save's subjects through `ikria-save` and joins them.
    ///
    /// # Errors
    ///
    /// The save reader's error when the subjects cannot be extracted.
    pub fn from_save(enumeration: &'e Enumeration, save: &Save) -> Result<Self, ExtractError> {
        Ok(Self::build(enumeration, &save.science_subjects()?))
    }

    /// One entry per subject of the enumeration, in its order.
    #[must_use]
    pub fn entries(&self) -> &[Entry<'e>] {
        &self.entries
    }

    /// The entry of one identifier, matched exactly.
    #[must_use]
    pub fn entry(&self, id: &str) -> Option<&Entry<'e>> {
        self.entries.iter().find(|e| e.subject.id() == id)
    }

    /// The save subjects that match no subject of the enumeration, such as asteroid
    /// samples and recoveries, in the save's order.
    #[must_use]
    pub fn unmatched(&self) -> &[SaveRecord] {
        &self.unmatched
    }

    /// The save subjects whose identifier repeats an earlier one, in the save's order.
    #[must_use]
    pub fn repeated(&self) -> &[SaveRecord] {
        &self.repeated
    }

    /// The entries whose save cap differs from the enumeration's value.
    pub fn cap_differences(&self) -> impl Iterator<Item = &Entry<'e>> {
        self.entries
            .iter()
            .filter(|e| matches!(e.cap_check(), CapCheck::Differs { .. }))
    }

    /// The totals of every entry.
    #[must_use]
    pub fn totals(&self) -> Totals {
        self.totals_where(|_| true)
    }

    /// The totals of the entries whose subject satisfies a predicate.
    #[must_use]
    pub fn totals_where(&self, predicate: impl Fn(&Subject) -> bool) -> Totals {
        let mut totals = Totals::default();
        for entry in self.entries.iter().filter(|e| predicate(e.subject)) {
            totals.add(entry);
        }
        totals
    }

    /// The totals by body, in the order of [`Body::ALL`].
    #[must_use]
    pub fn by_body(&self) -> Vec<(Body, Totals)> {
        Body::ALL
            .into_iter()
            .map(|b| (b, self.totals_where(|s| s.body() == b)))
            .collect()
    }

    /// The totals by situation, in the order of [`Situation::ALL`].
    #[must_use]
    pub fn by_situation(&self) -> Vec<(Situation, Totals)> {
        Situation::ALL
            .into_iter()
            .map(|sit| (sit, self.totals_where(|s| s.situation() == sit)))
            .collect()
    }

    /// The totals by experiment identifier, in alphabetical order. Each surface feature
    /// is its own experiment.
    #[must_use]
    pub fn by_experiment(&self) -> Vec<(&'static str, Totals)> {
        let mut groups: BTreeMap<&'static str, Totals> = BTreeMap::new();
        for entry in &self.entries {
            groups
                .entry(entry.subject.source().id())
                .or_default()
                .add(entry);
        }
        groups.into_iter().collect()
    }

    /// The totals by the rule each subject comes from.
    #[must_use]
    pub fn by_kind(&self) -> [(SubjectKind, Totals); 3] {
        [
            SubjectKind::Body,
            SubjectKind::SpaceCenterLocation,
            SubjectKind::SurfaceFeature,
        ]
        .map(|k| (k, self.totals_where(|s| s.kind() == k)))
    }

    /// The entries whose save multiplier differs from the enumeration's.
    pub fn multiplier_differences(&self) -> impl Iterator<Item = &Entry<'e>> {
        self.entries
            .iter()
            .filter(|e| matches!(e.multiplier_check(), MultiplierCheck::Differs { .. }))
    }

    /// The science the save holds in subjects that match nothing, in base units.
    #[must_use]
    pub fn unmatched_collected(&self) -> f64 {
        // A fold from positive zero, since an empty `sum` of floats is negative zero.
        self.unmatched.iter().fold(0.0, |sum, u| sum + u.collected)
    }

    /// The science the save holds in repeated records, in base units. The entries join
    /// only the first record of an identifier, so this science is in no entry's total.
    #[must_use]
    pub fn repeated_collected(&self) -> f64 {
        self.repeated.iter().fold(0.0, |sum, r| sum + r.collected)
    }

    /// An account of all the science the save holds, split by where the ledger puts it.
    #[must_use]
    pub fn save_account(&self) -> SaveAccount {
        let matched = self.totals();
        SaveAccount {
            records: matched
                .in_save
                .saturating_add(self.unmatched.len())
                .saturating_add(self.repeated.len()),
            matched_records: matched.in_save,
            matched: matched.collected,
            unmatched: self.unmatched_collected(),
            repeated: self.repeated_collected(),
        }
    }
}

/// All the science a save holds, in base units, split into the part joined to the
/// enumeration, the part in unmatched records and the part in repeated records. The three
/// parts sum to the save's whole, so no save science is missing from the ledger.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SaveAccount {
    /// The number of science records in the save.
    pub records: usize,
    /// How many of them were joined to the enumeration.
    pub matched_records: usize,
    /// The science in the joined records.
    pub matched: f64,
    /// The science in records that match nothing.
    pub unmatched: f64,
    /// The science in records whose identifier repeats an earlier one.
    pub repeated: f64,
}

impl SaveAccount {
    /// The whole of the save's science.
    #[must_use]
    pub fn total(&self) -> f64 {
        self.matched + self.unmatched + self.repeated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_refuses_zero_negative_and_non_finite() {
        for bad in [0.0, -0.6, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(ScienceScale::new(bad).is_none(), "{bad}");
        }
        assert_eq!(ScienceScale::new(0.6), Some(ScienceScale::HARD));
    }

    #[test]
    fn cap_comparison_tolerates_single_precision_only() {
        assert_eq!(CapCheck::compare(9.099_999_43, 9.1), CapCheck::Agrees);
        assert_eq!(CapCheck::compare(0.0, 0.0), CapCheck::Agrees);
        assert_eq!(
            CapCheck::compare(5.46, 9.1),
            CapCheck::Differs {
                save: 5.46,
                enumerated: 9.1
            }
        );
        assert!(matches!(
            CapCheck::compare(9.1 + 0.001, 9.1),
            CapCheck::Differs { .. }
        ));
    }
}
