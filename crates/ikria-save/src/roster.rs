//! The crew roster, the `KERBAL` nodes of the save's `ROSTER` node.

use crate::field::{
    ExtractError, Finite, child, non_empty, one_of, optional_finite, optional_unsigned,
};
use crate::node::Node;

/// A kerbal's place in the roster, from `type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KerbalType {
    /// Hired crew.
    Crew,
    /// An applicant who may be hired.
    Applicant,
    /// A kerbal not yet in the agency, such as one awaiting rescue.
    Unowned,
    /// A tourist.
    Tourist,
}

const KERBAL_TYPES: [(&str, KerbalType); 4] = [
    ("Crew", KerbalType::Crew),
    ("Applicant", KerbalType::Applicant),
    ("Unowned", KerbalType::Unowned),
    ("Tourist", KerbalType::Tourist),
];

/// A kerbal's roster status, from `state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RosterStatus {
    /// At the space centre and free to fly.
    Available,
    /// Aboard a vessel, or outside one.
    Assigned,
    /// Killed.
    Dead,
    /// Lost, and in a career returned after a time if the game's settings allow.
    Missing,
}

const ROSTER_STATUSES: [(&str, RosterStatus); 4] = [
    ("Available", RosterStatus::Available),
    ("Assigned", RosterStatus::Assigned),
    ("Dead", RosterStatus::Dead),
    ("Missing", RosterStatus::Missing),
];

/// A kerbal's trait, from `trait`. A trait outside the stock four, such as one a
/// modification adds, is kept by name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Trait {
    /// A pilot.
    Pilot,
    /// An engineer.
    Engineer,
    /// A scientist.
    Scientist,
    /// A tourist.
    Tourist,
    /// Any other trait, by name.
    Other(String),
}

impl Trait {
    fn from_save(name: &str) -> Self {
        match name {
            "Pilot" => Self::Pilot,
            "Engineer" => Self::Engineer,
            "Scientist" => Self::Scientist,
            "Tourist" => Self::Tourist,
            other => Self::Other(other.to_owned()),
        }
    }
}

/// One kerbal in the roster.
#[derive(Debug, Clone, PartialEq)]
pub struct Kerbal {
    /// The kerbal's name.
    pub name: String,
    /// The kerbal's place in the roster.
    pub kerbal_type: KerbalType,
    /// The kerbal's trait.
    pub kerbal_trait: Trait,
    /// The kerbal's status.
    pub status: RosterStatus,
    /// Experience from `experience`, where present. A save of game version 1.12.5 was
    /// not observed to write it, since the game derives experience from the career log.
    pub experience: Option<Finite>,
    /// The experience level from `experienceLevel`, where present, with the same caveat.
    pub experience_level: Option<u32>,
    /// Extra experience from `extraXP`, where present.
    pub extra_xp: Option<Finite>,
    /// The number of flights in the career log, from `CAREER_LOG`'s `flight`, where
    /// present.
    pub career_flights: Option<u32>,
}

/// Reads every `KERBAL` node of the `ROSTER` node.
///
/// # Errors
///
/// A missing `ROSTER`, or a kerbal without a valid `name`, `type`, `trait` or `state`,
/// or with an optional experience field that is not a number of the expected kind.
pub fn roster(game: &Node) -> Result<Vec<Kerbal>, ExtractError> {
    child(game, "ROSTER")?
        .children_named("KERBAL")
        .map(kerbal)
        .collect()
}

fn kerbal(node: &Node) -> Result<Kerbal, ExtractError> {
    let career_flights = match node.child("CAREER_LOG") {
        Some(log) => optional_unsigned(log, "flight")?,
        None => None,
    };
    Ok(Kerbal {
        name: non_empty(node, "name")?,
        kerbal_type: one_of(
            node,
            "type",
            &KERBAL_TYPES,
            "Crew, Applicant, Unowned or Tourist",
        )?,
        kerbal_trait: Trait::from_save(&non_empty(node, "trait")?),
        status: one_of(
            node,
            "state",
            &ROSTER_STATUSES,
            "Available, Assigned, Dead or Missing",
        )?,
        experience: optional_finite(node, "experience")?,
        experience_level: optional_unsigned(node, "experienceLevel")?,
        extra_xp: optional_finite(node, "extraXP")?,
        career_flights,
    })
}
