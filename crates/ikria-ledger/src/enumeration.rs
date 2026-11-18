//! Every subject the Phase 0 count counts, one by one, with the identifier the game
//! writes in a save and its value.
//!
//! # Rules
//!
//! The rules are those of `tools/phase0/science_subjects.py`, applied subject by subject
//! instead of by count.
//!
//! 1. **Body subjects.** For each body, each experiment and each situation, a subject
//!    exists when the body has a multiplier for the situation, the experiment's
//!    situation mask includes it, and the body has an atmosphere if the experiment needs
//!    one. When the experiment's biome mask includes the situation, there is one subject
//!    per biome, or one without a biome at a body that has none. Splashed subjects exist
//!    only where the multiplier table has a splashed value, which is Kerbin, Eve and
//!    Laythe, and they exist for every biome there, including biomes with no water.
//! 2. **Location subjects.** For each Space Center location biome, one landed subject per
//!    experiment whose situation and biome masks both include landed, at Kerbin's landed
//!    multiplier. The script does not test the atmosphere here, and neither does this
//!    port. Kerbin has one, so the result is the same.
//! 3. **Surface features.** One landed subject without a biome per feature, at its own
//!    cap and a multiplier of one. The multiplier of one is an unverified assumption,
//!    described at [`crate::catalog::SURFACE_FEATURES`].
//!
//! A subject's value before the difficulty multiplier is its cap times its multiplier,
//! and its Hard value is that times 0.6. Both are exact, in hundredths of a point.
//!
//! # Identifiers
//!
//! The identifier is `experiment@BodySituationBiome`. The biome is omitted when the
//! subject does not depend on one, and otherwise has its spaces removed and nothing
//! else changed, so `R&D` keeps its ampersand. This is a fact for the forms observed in
//! real saves, listed in the crate's `README.md`, and an inference for the rest.

use std::collections::HashMap;

use core::fmt;

use crate::catalog::{
    Body, Experiment, HARD_TENTHS, LOCATION_BIOMES, Multiplier, Provenance, SURFACE_FEATURES,
    Situation, SurfaceFeature,
};

/// A quantity of science in exact hundredths of a point.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hundredths(u64);

impl Hundredths {
    /// Zero.
    pub const ZERO: Self = Self(0);

    /// Wraps a count of hundredths.
    #[must_use]
    pub const fn new(hundredths: u64) -> Self {
        Self(hundredths)
    }

    /// The count of hundredths.
    #[must_use]
    pub fn get(self) -> u64 {
        self.0
    }

    /// The quantity as a number of points.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_points(self) -> f64 {
        // Every value of the count is far below 2^53, so the integer converts exactly, but
        // most hundredths have no exact binary fraction, so the quotient is the nearest
        // double to the decimal value, not the value itself.
        self.0 as f64 / 100.0
    }

    /// The sum, or `None` on overflow, which no count of this size can reach.
    #[must_use]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    /// Rounded to tenths, with ties to even, as Python's `round(x, 1)` does on an exact
    /// decimal. The result is in tenths.
    #[must_use]
    pub fn round_to_tenths(self) -> u64 {
        round_half_even(self.0, 10)
    }

    /// Rounded to whole points, with ties to even, as Python's `round(x)` does on an exact
    /// decimal.
    #[must_use]
    pub fn round_to_points(self) -> u64 {
        round_half_even(self.0, 100)
    }
}

impl fmt::Display for Hundredths {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, self.0 % 100)
    }
}

/// Divides by a positive divisor and rounds half to even.
fn round_half_even(value: u64, divisor: u64) -> u64 {
    debug_assert!(
        divisor > 0,
        "the divisors used are the constants 10 and 100"
    );
    let quotient = value / divisor;
    let remainder = value % divisor;
    let twice = remainder.saturating_mul(2);
    if twice > divisor || (twice == divisor && quotient % 2 == 1) {
        quotient.saturating_add(1)
    } else {
        quotient
    }
}

/// Which rule of the count a subject comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubjectKind {
    /// Rule 1, an experiment at a body.
    Body,
    /// Rule 2, an experiment at a Space Center location biome.
    SpaceCenterLocation,
    /// Rule 3, a surface feature.
    SurfaceFeature,
}

/// The experiment a subject belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    /// One of the thirteen experiments of the script's table.
    Experiment(Experiment),
    /// A surface feature.
    SurfaceFeature(&'static SurfaceFeature),
}

impl Source {
    /// The experiment identifier the game writes before the `@`.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::Experiment(e) => e.id(),
            Self::SurfaceFeature(f) => f.id,
        }
    }

    /// The cap before any multiplier.
    #[must_use]
    pub fn cap(self) -> u16 {
        match self {
            Self::Experiment(e) => e.cap(),
            Self::SurfaceFeature(f) => f.cap,
        }
    }
}

/// A biome of a subject, with the provenance of its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Biome {
    /// The name with spaces, as its source writes it.
    pub name: &'static str,
    /// Where the name comes from.
    pub provenance: Provenance,
}

/// One science subject of the count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subject {
    id: String,
    kind: SubjectKind,
    source: Source,
    body: Body,
    situation: Situation,
    biome: Option<Biome>,
    multiplier: Multiplier,
}

impl Subject {
    fn new(
        kind: SubjectKind,
        source: Source,
        body: Body,
        situation: Situation,
        biome: Option<Biome>,
        multiplier: Multiplier,
    ) -> Self {
        let mut id = format!("{}@{}{}", source.id(), body.name(), situation.id());
        if let Some(b) = biome {
            id.extend(b.name.chars().filter(|c| *c != ' '));
        }
        Self {
            id,
            kind,
            source,
            body,
            situation,
            biome,
            multiplier,
        }
    }

    /// The identifier the game writes in a save.
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The rule the subject comes from.
    #[must_use]
    pub fn kind(&self) -> SubjectKind {
        self.kind
    }

    /// The experiment.
    #[must_use]
    pub fn source(&self) -> Source {
        self.source
    }

    /// The body.
    #[must_use]
    pub fn body(&self) -> Body {
        self.body
    }

    /// The situation.
    #[must_use]
    pub fn situation(&self) -> Situation {
        self.situation
    }

    /// The biome, where the subject depends on one.
    #[must_use]
    pub fn biome(&self) -> Option<Biome> {
        self.biome
    }

    /// Whether the identifier holds a placeholder for an unnamed location, so that it
    /// cannot match a save.
    #[must_use]
    pub fn is_placeholder(&self) -> bool {
        self.biome
            .is_some_and(|b| b.provenance == Provenance::Placeholder)
    }

    /// The multiplier of the body and situation, or one for a surface feature.
    #[must_use]
    pub fn multiplier(&self) -> Multiplier {
        self.multiplier
    }

    /// Whether the multiplier is an unverified assumption, which holds for the surface
    /// features.
    #[must_use]
    pub fn multiplier_is_assumed(&self) -> bool {
        self.kind == SubjectKind::SurfaceFeature
    }

    /// Whether the identifier relies on a biome name that no save has confirmed, a
    /// localization display name or a placeholder.
    #[must_use]
    pub fn relies_on_unverified_name(&self) -> bool {
        self.biome.is_some_and(|b| !b.provenance.is_verified())
    }

    /// The value before the difficulty multiplier, the cap times the multiplier. This is
    /// the scale of the `cap` and `sci` fields of a save, an inference the README explains.
    #[must_use]
    pub fn base_value(&self) -> Hundredths {
        // Cap times tenths is in tenths, and times ten is in hundredths.
        Hundredths(u64::from(self.source.cap()) * u64::from(self.multiplier.tenths()) * 10)
    }

    /// The value on Hard, the base value times 0.6.
    #[must_use]
    pub fn hard_value(&self) -> Hundredths {
        // Cap times tenths times tenths is in hundredths.
        Hundredths(
            u64::from(self.source.cap())
                * u64::from(self.multiplier.tenths())
                * u64::from(HARD_TENTHS),
        )
    }
}

/// Why the enumeration could not be built. Only a defect in the tables can cause it,
/// and a test proves that the tables have none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumerationError {
    /// Two subjects have the same identifier.
    DuplicateId(String),
    /// A body in the surface feature table has no landed multiplier.
    NoLandedMultiplier(Body),
}

impl fmt::Display for EnumerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(f, "two subjects have the identifier `{id}`"),
            Self::NoLandedMultiplier(b) => write!(f, "{b} has no landed multiplier"),
        }
    }
}

impl std::error::Error for EnumerationError {}

/// A count and a sum of values, for a group of subjects.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CountTotals {
    /// The number of subjects.
    pub subjects: usize,
    /// The sum of their base values.
    pub base: Hundredths,
    /// The sum of their Hard values.
    pub hard: Hundredths,
}

impl CountTotals {
    fn add(&mut self, subject: &Subject) {
        // The totals of 3,104 subjects of at most a few hundred points each cannot
        // overflow 64 bits, so saturation never takes effect.
        self.subjects = self.subjects.saturating_add(1);
        self.base = self
            .base
            .checked_add(subject.base_value())
            .unwrap_or(Hundredths(u64::MAX));
        self.hard = self
            .hard
            .checked_add(subject.hard_value())
            .unwrap_or(Hundredths(u64::MAX));
    }
}

/// Every subject of the count, in a fixed order, with an index by identifier.
#[derive(Debug, Clone)]
pub struct Enumeration {
    subjects: Vec<Subject>,
    index: HashMap<String, usize>,
}

impl Enumeration {
    /// Builds the enumeration under the rules above. The order is the body subjects by
    /// body, experiment, situation and biome, then the location subjects, then the
    /// surface features.
    ///
    /// # Errors
    ///
    /// An [`EnumerationError`] if the tables are defective.
    pub fn phase0() -> Result<Self, EnumerationError> {
        let mut subjects = Vec::new();
        push_body_subjects(&mut subjects);
        push_location_subjects(&mut subjects)?;
        push_surface_features(&mut subjects, &SURFACE_FEATURES)?;
        let mut index = HashMap::with_capacity(subjects.len());
        for (i, subject) in subjects.iter().enumerate() {
            if index.insert(subject.id.clone(), i).is_some() {
                return Err(EnumerationError::DuplicateId(subject.id.clone()));
            }
        }
        Ok(Self { subjects, index })
    }

    /// Every subject, in the order described at [`Enumeration::phase0`].
    #[must_use]
    pub fn subjects(&self) -> &[Subject] {
        &self.subjects
    }

    /// The subject with this identifier, matched exactly, including case.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Subject> {
        self.index.get(id).and_then(|&i| self.subjects.get(i))
    }

    /// The number of subjects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.subjects.len()
    }

    /// Whether there are no subjects, which never holds for the count.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subjects.is_empty()
    }

    /// The totals of the subjects that satisfy a predicate.
    #[must_use]
    pub fn totals_where(&self, predicate: impl Fn(&Subject) -> bool) -> CountTotals {
        let mut totals = CountTotals::default();
        for subject in self.subjects.iter().filter(|s| predicate(s)) {
            totals.add(subject);
        }
        totals
    }

    /// The totals of every subject.
    #[must_use]
    pub fn totals(&self) -> CountTotals {
        self.totals_where(|_| true)
    }

    /// The number of subjects whose identifier relies on an unverified biome name.
    #[must_use]
    pub fn unverified_name_count(&self) -> usize {
        self.subjects
            .iter()
            .filter(|s| s.relies_on_unverified_name())
            .count()
    }

    /// The number of subjects whose multiplier is an unverified assumption.
    #[must_use]
    pub fn assumed_multiplier_count(&self) -> usize {
        self.subjects
            .iter()
            .filter(|s| s.multiplier_is_assumed())
            .count()
    }
}

fn push_body_subjects(out: &mut Vec<Subject>) {
    for body in Body::ALL {
        for experiment in Experiment::ALL {
            if experiment.requires_atmosphere() && !body.has_atmosphere() {
                continue;
            }
            for situation in Situation::ALL {
                let Some(multiplier) = body.multiplier(situation) else {
                    continue;
                };
                if !experiment.allows(situation) {
                    continue;
                }
                let source = Source::Experiment(experiment);
                if experiment.biome_dependent(situation) && !body.biomes().is_empty() {
                    for &name in body.biomes() {
                        let biome = Biome {
                            name,
                            provenance: Provenance::Inventory,
                        };
                        out.push(Subject::new(
                            SubjectKind::Body,
                            source,
                            body,
                            situation,
                            Some(biome),
                            multiplier,
                        ));
                    }
                } else {
                    out.push(Subject::new(
                        SubjectKind::Body,
                        source,
                        body,
                        situation,
                        None,
                        multiplier,
                    ));
                }
            }
        }
    }
}

fn push_location_subjects(out: &mut Vec<Subject>) -> Result<(), EnumerationError> {
    let landed = Situation::SrfLanded;
    let multiplier = Body::Kerbin
        .multiplier(landed)
        .ok_or(EnumerationError::NoLandedMultiplier(Body::Kerbin))?;
    for location in &LOCATION_BIOMES {
        for experiment in Experiment::ALL {
            if !(experiment.allows(landed) && experiment.biome_dependent(landed)) {
                continue;
            }
            let biome = Biome {
                name: location.name,
                provenance: location.provenance,
            };
            out.push(Subject::new(
                SubjectKind::SpaceCenterLocation,
                Source::Experiment(experiment),
                Body::Kerbin,
                landed,
                Some(biome),
                multiplier,
            ));
        }
    }
    Ok(())
}

fn push_surface_features(
    out: &mut Vec<Subject>,
    features: &'static [SurfaceFeature],
) -> Result<(), EnumerationError> {
    for feature in features {
        // A feature stands on a surface, so its body must have a landed situation.
        if feature.body.multiplier(Situation::SrfLanded).is_none() {
            return Err(EnumerationError::NoLandedMultiplier(feature.body));
        }
        out.push(Subject::new(
            SubjectKind::SurfaceFeature,
            Source::SurfaceFeature(feature),
            feature.body,
            Situation::SrfLanded,
            None,
            Multiplier::ONE,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounding_is_half_to_even() {
        assert_eq!(Hundredths::new(123_876).round_to_points(), 1239);
        assert_eq!(Hundredths::new(250).round_to_points(), 2);
        assert_eq!(Hundredths::new(350).round_to_points(), 4);
        assert_eq!(Hundredths::new(249).round_to_points(), 2);
        assert_eq!(Hundredths::new(251).round_to_points(), 3);
        assert_eq!(Hundredths::new(0).round_to_points(), 0);
        assert_eq!(Hundredths::new(125).round_to_tenths(), 12);
        assert_eq!(Hundredths::new(135).round_to_tenths(), 14);
        assert_eq!(Hundredths::new(u64::MAX).round_to_points(), u64::MAX / 100);
        assert_eq!(
            Hundredths::new(u64::MAX - 15).round_to_tenths(),
            (u64::MAX - 15) / 10
        );
    }

    #[test]
    fn display_shows_two_decimals() {
        assert_eq!(Hundredths::new(90).to_string(), "0.90");
        assert_eq!(Hundredths::new(184_170).to_string(), "1841.70");
    }

    #[test]
    fn surface_feature_body_without_landing_is_refused() {
        static FEATURES: [SurfaceFeature; 1] = [SurfaceFeature {
            id: "ROCScience_Test",
            body: Body::Jool,
            cap: 30,
        }];
        let mut out = Vec::new();
        assert_eq!(
            push_surface_features(&mut out, &FEATURES),
            Err(EnumerationError::NoLandedMultiplier(Body::Jool))
        );
        assert!(out.is_empty());
    }
}
