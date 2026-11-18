//! The agency's totals, its facility levels, its researched technologies and its science
//! subjects, all read from the save's `SCENARIO` nodes.

use crate::field::{
    ExtractError, Finite, NonNegative, finite, invalid, missing_node, non_empty, non_negative,
    one_of, optional_bool, required, text, unsigned,
};
use crate::node::Node;

/// Finds the first `SCENARIO` node whose `name` is the one given.
///
/// # Errors
///
/// [`ExtractError::MissingNode`] when no such scenario exists.
pub fn scenario<'a>(game: &'a Node, name: &str) -> Result<&'a Node, ExtractError> {
    game.children_named("SCENARIO")
        .find(|s| s.value("name").is_some_and(|v| v.value() == name))
        .ok_or_else(|| missing_node(format!("SCENARIO name = {name}"), game))
}

/// The agency's three totals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Agency {
    /// Funds, from the `Funding` scenario's `funds`. The game allows a negative balance
    /// through penalties *(inference)*, so it is not required to be positive.
    pub funds: Finite,
    /// Science points, from the `ResearchAndDevelopment` scenario's `sci`.
    pub science: Finite,
    /// Reputation, from the `Reputation` scenario's `rep`, which may be negative.
    pub reputation: Finite,
}

/// Reads the agency's totals. The three scenarios exist in a career save and not in a
/// sandbox save, so a sandbox save is refused.
///
/// # Errors
///
/// A missing scenario or field, or a value that is not a finite number.
pub fn agency(game: &Node) -> Result<Agency, ExtractError> {
    Ok(Agency {
        funds: finite(scenario(game, "Funding")?, "funds")?,
        science: finite(scenario(game, "ResearchAndDevelopment")?, "sci")?,
        reputation: finite(scenario(game, "Reputation")?, "rep")?,
    })
}

/// A stock upgradeable facility at the space centre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facility {
    /// `SpaceCenter/LaunchPad`.
    LaunchPad,
    /// `SpaceCenter/Runway`.
    Runway,
    /// `SpaceCenter/VehicleAssemblyBuilding`.
    VehicleAssemblyBuilding,
    /// `SpaceCenter/SpaceplaneHangar`.
    SpaceplaneHangar,
    /// `SpaceCenter/TrackingStation`.
    TrackingStation,
    /// `SpaceCenter/AstronautComplex`.
    AstronautComplex,
    /// `SpaceCenter/MissionControl`.
    MissionControl,
    /// `SpaceCenter/ResearchAndDevelopment`.
    ResearchAndDevelopment,
    /// `SpaceCenter/Administration`.
    Administration,
    /// `SpaceCenter/FlagPole`.
    FlagPole,
}

const FACILITIES: [(&str, Facility); 10] = [
    ("SpaceCenter/LaunchPad", Facility::LaunchPad),
    ("SpaceCenter/Runway", Facility::Runway),
    (
        "SpaceCenter/VehicleAssemblyBuilding",
        Facility::VehicleAssemblyBuilding,
    ),
    ("SpaceCenter/SpaceplaneHangar", Facility::SpaceplaneHangar),
    ("SpaceCenter/TrackingStation", Facility::TrackingStation),
    ("SpaceCenter/AstronautComplex", Facility::AstronautComplex),
    ("SpaceCenter/MissionControl", Facility::MissionControl),
    (
        "SpaceCenter/ResearchAndDevelopment",
        Facility::ResearchAndDevelopment,
    ),
    ("SpaceCenter/Administration", Facility::Administration),
    ("SpaceCenter/FlagPole", Facility::FlagPole),
];

/// A facility's level, as an index from 0 for the first level to 2 for the third.
///
/// # Conversion rule
///
/// The save stores a level as a fraction of the highest, `lvl = index / (levels - 1)`.
/// This is an inference from the observed values and the game's modding documentation,
/// not verified against its code. Every stock facility is taken to have three levels, so
/// the fractions 0, 0.5 and 1 are the indices 0, 1 and 2, and the game's displayed level
/// is the index plus one. A fraction more than [`FacilityLevel::TOLERANCE`] from one of
/// those three is refused rather than rounded, because it would mean that the
/// three-level assumption is wrong for that facility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FacilityLevel(u8);

impl FacilityLevel {
    /// The number of levels every stock facility is assumed to have.
    pub const LEVELS: u8 = 3;

    /// How far a stored fraction may sit from an exact level, to allow for the single
    /// precision the game stores some numbers in.
    pub const TOLERANCE: f64 = 1e-4;

    /// Wraps a level index, which must be below [`FacilityLevel::LEVELS`].
    #[must_use]
    pub fn new(index: u8) -> Option<Self> {
        (index < Self::LEVELS).then_some(Self(index))
    }

    /// Converts a stored fraction under the rule above.
    #[must_use]
    pub fn from_fraction(fraction: f64) -> Option<Self> {
        if !fraction.is_finite() {
            return None;
        }
        let scaled = fraction * f64::from(Self::LEVELS - 1);
        (0..Self::LEVELS).find_map(|index| {
            ((scaled - f64::from(index)).abs() <= Self::TOLERANCE).then_some(Self(index))
        })
    }

    /// The index, from 0.
    #[must_use]
    pub fn index(self) -> u8 {
        self.0
    }

    /// The level as the game displays it, from 1.
    #[must_use]
    pub fn displayed(self) -> u8 {
        self.0 + 1
    }
}

/// One facility and its level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FacilityRecord {
    /// The facility.
    pub facility: Facility,
    /// Its level.
    pub level: FacilityLevel,
}

/// Reads the stock facilities' levels from `ScenarioUpgradeableFacilities`, in the order
/// the save lists them. A child node whose name is not a stock facility, such as one a
/// modification adds, is ignored like an unknown field.
///
/// # Errors
///
/// A missing scenario, a stock facility without `lvl`, or a `lvl` that is not one of the
/// three fractions.
pub fn facilities(game: &Node) -> Result<Vec<FacilityRecord>, ExtractError> {
    let node = scenario(game, "ScenarioUpgradeableFacilities")?;
    let mut out = Vec::new();
    for child in node.children() {
        let Some((_, facility)) = FACILITIES.iter().find(|(name, _)| *name == child.name()) else {
            continue;
        };
        let value = required(child, "lvl")?;
        let level = value
            .value()
            .parse::<f64>()
            .ok()
            .and_then(FacilityLevel::from_fraction)
            .ok_or_else(|| invalid(child, "lvl", value, "0, 0.5 or 1"))?;
        out.push(FacilityRecord {
            facility: *facility,
            level,
        });
    }
    Ok(out)
}

/// The research state of a technology node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TechState {
    /// Researched.
    Available,
    /// Not researched.
    Unavailable,
}

const TECH_STATES: [(&str, TechState); 2] = [
    ("Available", TechState::Available),
    ("Unavailable", TechState::Unavailable),
];

/// A technology node from the `ResearchAndDevelopment` scenario.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Technology {
    /// The node's identifier, such as `basicRocketry`.
    pub id: String,
    /// Whether it is researched.
    pub state: TechState,
    /// Its cost in science points.
    pub cost: u32,
    /// The parts purchased in it, as the save lists them. A part may appear more than
    /// once, as observed in a save of game version 1.12.5, and the list is kept as
    /// written rather than deduplicated.
    pub parts: Vec<String>,
}

/// Reads every `Tech` node of the `ResearchAndDevelopment` scenario.
///
/// # Errors
///
/// A missing scenario, or a `Tech` node without a valid `id`, `state` or `cost`.
pub fn technologies(game: &Node) -> Result<Vec<Technology>, ExtractError> {
    let node = scenario(game, "ResearchAndDevelopment")?;
    node.children_named("Tech")
        .map(|tech| {
            Ok(Technology {
                id: non_empty(tech, "id")?,
                state: one_of(tech, "state", &TECH_STATES, "Available or Unavailable")?,
                cost: unsigned(tech, "cost")?,
                parts: tech
                    .values_named("part")
                    .map(|v| v.value().to_owned())
                    .collect(),
            })
        })
        .collect()
}

/// A science subject, one `Science` node of the `ResearchAndDevelopment` scenario.
///
/// The meanings of the abbreviated keys are *inferences* from their values and from the
/// game's modding documentation. Every number is finite and not negative.
#[derive(Debug, Clone, PartialEq)]
pub struct ScienceSubject {
    /// The subject's identifier, `experiment@situation` with body and biome, from `id`.
    pub id: String,
    /// The subject's title, from `title`, which may be empty.
    pub title: String,
    /// Science collected so far, from `sci`.
    pub science: NonNegative,
    /// The most science the subject can yield, from `cap`.
    pub cap: NonNegative,
    /// The data scale, from `dsc`.
    pub data_scale: NonNegative,
    /// The subject value, the multiplier of its situation and body, from `sbv`.
    pub subject_value: NonNegative,
    /// The scientific value remaining as a fraction, from `scv`.
    pub scientific_value: NonNegative,
    /// Whether the game's science scale applies, from `asc`, where present.
    pub apply_science_scale: Option<bool>,
}

/// Reads every `Science` node of the `ResearchAndDevelopment` scenario.
///
/// # Errors
///
/// A missing scenario, or a subject without a valid `id`, `title`, `sci`, `cap`, `dsc`,
/// `sbv` or `scv`, or with an `asc` that is not a boolean.
pub fn science_subjects(game: &Node) -> Result<Vec<ScienceSubject>, ExtractError> {
    let node = scenario(game, "ResearchAndDevelopment")?;
    node.children_named("Science")
        .map(|subject| {
            Ok(ScienceSubject {
                id: non_empty(subject, "id")?,
                title: text(subject, "title")?,
                science: non_negative(subject, "sci")?,
                cap: non_negative(subject, "cap")?,
                data_scale: non_negative(subject, "dsc")?,
                subject_value: non_negative(subject, "sbv")?,
                scientific_value: non_negative(subject, "scv")?,
                apply_science_scale: optional_bool(subject, "asc")?,
            })
        })
        .collect()
}
