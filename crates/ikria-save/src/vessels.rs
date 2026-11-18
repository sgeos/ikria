//! Vessels, the `VESSEL` nodes of the save's `FLIGHTSTATE` node. A kerbal outside a
//! vessel and a planted flag are vessels in the save, of the types `EVA` and `Flag`.

use crate::field::{
    ExtractError, child, invalid, non_empty, one_of, optional_text, required, unsigned, unsigned_of,
};
use crate::node::Node;

/// A vessel's type, from `type`, the closed set of game version 1.12.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VesselType {
    /// Debris.
    Debris,
    /// An asteroid or comet.
    SpaceObject,
    /// An unidentified object.
    Unknown,
    /// An uncrewed probe.
    Probe,
    /// A communications relay.
    Relay,
    /// A rover.
    Rover,
    /// A lander.
    Lander,
    /// A ship.
    Ship,
    /// A plane.
    Plane,
    /// A station.
    Station,
    /// A base.
    Base,
    /// A kerbal outside a vessel, written `EVA`.
    Eva,
    /// A planted flag.
    Flag,
    /// The controller of a deployed science station.
    DeployedScienceController,
    /// A deployed science experiment.
    DeployedSciencePart,
    /// A part dropped from a kerbal's inventory.
    DroppedPart,
}

const VESSEL_TYPES: [(&str, VesselType); 16] = [
    ("Debris", VesselType::Debris),
    ("SpaceObject", VesselType::SpaceObject),
    ("Unknown", VesselType::Unknown),
    ("Probe", VesselType::Probe),
    ("Relay", VesselType::Relay),
    ("Rover", VesselType::Rover),
    ("Lander", VesselType::Lander),
    ("Ship", VesselType::Ship),
    ("Plane", VesselType::Plane),
    ("Station", VesselType::Station),
    ("Base", VesselType::Base),
    ("EVA", VesselType::Eva),
    ("Flag", VesselType::Flag),
    (
        "DeployedScienceController",
        VesselType::DeployedScienceController,
    ),
    ("DeployedSciencePart", VesselType::DeployedSciencePart),
    ("DroppedPart", VesselType::DroppedPart),
];

/// A vessel's situation, from `sit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VesselSituation {
    /// On the ground, written `LANDED`.
    Landed,
    /// In water, written `SPLASHED`.
    Splashed,
    /// On the launch pad or runway before launch, written `PRELAUNCH`.
    Prelaunch,
    /// In an atmosphere, written `FLYING`.
    Flying,
    /// On a suborbital path, written `SUB_ORBITAL`.
    SubOrbital,
    /// In orbit, written `ORBITING`.
    Orbiting,
    /// On an escape path, written `ESCAPING`.
    Escaping,
    /// Docked to another vessel, written `DOCKED`.
    Docked,
}

const SITUATIONS: [(&str, VesselSituation); 8] = [
    ("LANDED", VesselSituation::Landed),
    ("SPLASHED", VesselSituation::Splashed),
    ("PRELAUNCH", VesselSituation::Prelaunch),
    ("FLYING", VesselSituation::Flying),
    ("SUB_ORBITAL", VesselSituation::SubOrbital),
    ("ORBITING", VesselSituation::Orbiting),
    ("ESCAPING", VesselSituation::Escaping),
    ("DOCKED", VesselSituation::Docked),
];

/// A celestial body by the index the save stores in an orbit's `REF`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BodyIndex(pub u32);

/// The stock bodies by index. The order is an *inference*, corroborated by the reversed
/// order of the bodies in a 1.12.5 save's contract weights and by contracts that name
/// Kerbin as body 1. A modification that adds or reorders bodies invalidates it.
const STOCK_BODIES: [&str; 17] = [
    "Sun", "Kerbin", "Mun", "Minmus", "Moho", "Eve", "Duna", "Ike", "Jool", "Laythe", "Vall",
    "Bop", "Tylo", "Gilly", "Pol", "Dres", "Eeloo",
];

impl BodyIndex {
    /// The stock body's name, or `None` for an index past the stock system.
    #[must_use]
    pub fn stock_name(self) -> Option<&'static str> {
        usize::try_from(self.0)
            .ok()
            .and_then(|i| STOCK_BODIES.get(i))
            .copied()
    }
}

/// One vessel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vessel {
    /// The game's unique identifier, 32 hexadecimal digits, from `pid`.
    pub pid: String,
    /// The persistent identifier, from `persistentId`.
    pub persistent_id: u32,
    /// The vessel's name, from `name`. It may be a localisation tag such as
    /// `#autoLOC_501180`, which is kept as written. A kerbal outside a vessel carries
    /// the kerbal's name.
    pub name: String,
    /// The vessel's type.
    pub vessel_type: VesselType,
    /// The vessel's situation.
    pub situation: VesselSituation,
    /// The body it orbits or rests on, from `ORBIT`'s `REF`.
    pub body: BodyIndex,
    /// Where it landed, from `landedAt`, absent when the save leaves it empty.
    pub landed_at: Option<String>,
    /// The number of `PART` nodes.
    pub part_count: usize,
    /// The crew aboard, the `crew` values of every part in order.
    pub crew: Vec<String>,
}

impl Vessel {
    /// Whether this is a kerbal outside a vessel.
    #[must_use]
    pub fn is_eva(&self) -> bool {
        self.vessel_type == VesselType::Eva
    }

    /// Whether this is a planted flag.
    #[must_use]
    pub fn is_flag(&self) -> bool {
        self.vessel_type == VesselType::Flag
    }
}

/// Reads every `VESSEL` node of the `FLIGHTSTATE` node.
///
/// # Errors
///
/// A missing `FLIGHTSTATE`, or a vessel without a valid `pid`, `persistentId`, `name`,
/// `type`, `sit` or `ORBIT` with `REF`.
pub fn vessels(game: &Node) -> Result<Vec<Vessel>, ExtractError> {
    child(game, "FLIGHTSTATE")?
        .children_named("VESSEL")
        .map(vessel)
        .collect()
}

fn vessel(node: &Node) -> Result<Vessel, ExtractError> {
    let pid_value = required(node, "pid")?;
    let pid = pid_value.value();
    if pid.is_empty() || !pid.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(invalid(node, "pid", pid_value, "hexadecimal digits"));
    }
    let orbit = child(node, "ORBIT")?;
    let body = BodyIndex(unsigned_of(orbit, "REF", required(orbit, "REF")?)?);
    let parts: Vec<&Node> = node.children_named("PART").collect();
    Ok(Vessel {
        pid: pid.to_owned(),
        persistent_id: unsigned(node, "persistentId")?,
        name: non_empty(node, "name")?,
        vessel_type: one_of(node, "type", &VESSEL_TYPES, "a stock vessel type")?,
        situation: one_of(node, "sit", &SITUATIONS, "a stock vessel situation")?,
        body,
        landed_at: optional_text(node, "landedAt"),
        part_count: parts.len(),
        crew: parts
            .iter()
            .flat_map(|p| p.values_named("crew"))
            .map(|v| v.value().to_owned())
            .filter(|c| !c.is_empty())
            .collect(),
    })
}
