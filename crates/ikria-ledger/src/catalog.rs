//! The rules of the Phase 0 count, as typed tables.
//!
//! Every table here is a port of `tools/phase0/science_subjects.py` or of the tracked
//! inventory it read, except the names of the Space Center location biomes and the list
//! of surface features, which the script held only as counts. The tests in
//! `tests/phase0.rs` hold each ported table in agreement with its source.
//!
//! # Sources
//!
//! - **Experiments.** The caps and masks of the thirteen experiments, from the script's
//!   `EXPS`, which took them from the game's `ScienceDefs.cfg`.
//! - **Multipliers.** The body and situation multipliers, from the script's `MULT`, which
//!   took them from the official wiki.
//! - **Biomes and atmospheres.** From `record/verification/data/inventory.json`, which
//!   Phase 0 read from the running game.
//! - **Location biomes.** See [`LOCATION_BIOMES`] for the provenance of each name.
//! - **Surface features.** From Breaking Ground's `ScienceDefs.cfg`, read without change on
//!   2026-11-18. See [`SURFACE_FEATURES`].

use core::fmt;

/// A situation, as the game writes it in a subject identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Situation {
    /// On the surface, `SrfLanded`.
    SrfLanded,
    /// On water, `SrfSplashed`.
    SrfSplashed,
    /// In the lower atmosphere, `FlyingLow`.
    FlyingLow,
    /// In the upper atmosphere, `FlyingHigh`.
    FlyingHigh,
    /// In space near the body, `InSpaceLow`.
    InSpaceLow,
    /// In space far from the body, `InSpaceHigh`.
    InSpaceHigh,
}

impl Situation {
    /// Every situation, in the order of the game's mask bits.
    pub const ALL: [Self; 6] = [
        Self::SrfLanded,
        Self::SrfSplashed,
        Self::FlyingLow,
        Self::FlyingHigh,
        Self::InSpaceLow,
        Self::InSpaceHigh,
    ];

    /// The text the game writes in a subject identifier.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Self::SrfLanded => "SrfLanded",
            Self::SrfSplashed => "SrfSplashed",
            Self::FlyingLow => "FlyingLow",
            Self::FlyingHigh => "FlyingHigh",
            Self::InSpaceLow => "InSpaceLow",
            Self::InSpaceHigh => "InSpaceHigh",
        }
    }

    /// The situation's bit in an experiment's situation and biome masks.
    #[must_use]
    pub fn bit(self) -> u8 {
        match self {
            Self::SrfLanded => 1,
            Self::SrfSplashed => 2,
            Self::FlyingLow => 4,
            Self::FlyingHigh => 8,
            Self::InSpaceLow => 16,
            Self::InSpaceHigh => 32,
        }
    }

    fn index(self) -> usize {
        match self {
            Self::SrfLanded => 0,
            Self::SrfSplashed => 1,
            Self::FlyingLow => 2,
            Self::FlyingHigh => 3,
            Self::InSpaceLow => 4,
            Self::InSpaceHigh => 5,
        }
    }
}

impl fmt::Display for Situation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

/// A multiplier of a body and situation, held exactly in tenths, since every
/// multiplier of the count has at most one decimal place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Multiplier(u16);

impl Multiplier {
    /// A multiplier of exactly one, which surface features carry.
    pub const ONE: Self = Self(10);

    /// Wraps a multiplier given in tenths. Zero is refused, since a situation without
    /// science has no multiplier at all.
    #[must_use]
    pub const fn from_tenths(tenths: u16) -> Option<Self> {
        if tenths == 0 {
            None
        } else {
            Some(Self(tenths))
        }
    }

    /// The multiplier in tenths.
    #[must_use]
    pub fn tenths(self) -> u16 {
        self.0
    }

    /// The multiplier as a number.
    #[must_use]
    pub fn get(self) -> f64 {
        f64::from(self.0) / 10.0
    }
}

/// A shorthand for the multiplier tables, which hold only nonzero literals.
const fn m(tenths: u16) -> Option<Multiplier> {
    Multiplier::from_tenths(tenths)
}

/// A stock body that the count covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Body {
    /// The Sun.
    Sun,
    /// Moho.
    Moho,
    /// Eve.
    Eve,
    /// Gilly.
    Gilly,
    /// Kerbin.
    Kerbin,
    /// The Mun.
    Mun,
    /// Minmus.
    Minmus,
    /// Duna.
    Duna,
    /// Ike.
    Ike,
    /// Dres.
    Dres,
    /// Jool.
    Jool,
    /// Laythe.
    Laythe,
    /// Vall.
    Vall,
    /// Tylo.
    Tylo,
    /// Bop.
    Bop,
    /// Pol.
    Pol,
    /// Eeloo.
    Eeloo,
}

impl Body {
    /// Every body, in the order of the script's `MULT` table.
    pub const ALL: [Self; 17] = [
        Self::Sun,
        Self::Moho,
        Self::Eve,
        Self::Gilly,
        Self::Kerbin,
        Self::Mun,
        Self::Minmus,
        Self::Duna,
        Self::Ike,
        Self::Dres,
        Self::Jool,
        Self::Laythe,
        Self::Vall,
        Self::Tylo,
        Self::Bop,
        Self::Pol,
        Self::Eeloo,
    ];

    /// The body's name as the game writes it in a subject identifier.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Sun => "Sun",
            Self::Moho => "Moho",
            Self::Eve => "Eve",
            Self::Gilly => "Gilly",
            Self::Kerbin => "Kerbin",
            Self::Mun => "Mun",
            Self::Minmus => "Minmus",
            Self::Duna => "Duna",
            Self::Ike => "Ike",
            Self::Dres => "Dres",
            Self::Jool => "Jool",
            Self::Laythe => "Laythe",
            Self::Vall => "Vall",
            Self::Tylo => "Tylo",
            Self::Bop => "Bop",
            Self::Pol => "Pol",
            Self::Eeloo => "Eeloo",
        }
    }

    /// Whether the body has an atmosphere, from the inventory. The Sun has one there.
    #[must_use]
    pub fn has_atmosphere(self) -> bool {
        matches!(
            self,
            Self::Sun | Self::Eve | Self::Kerbin | Self::Duna | Self::Jool | Self::Laythe
        )
    }

    /// The body's biomes as the inventory names them, with spaces. The Sun and Jool have
    /// none.
    #[must_use]
    pub fn biomes(self) -> &'static [&'static str] {
        match self {
            Self::Sun | Self::Jool => &[],
            Self::Moho => &[
                "Canyon",
                "Central Lowlands",
                "Highlands",
                "Midlands",
                "Minor Craters",
                "North Pole",
                "Northern Sinkhole",
                "Northern Sinkhole Ridge",
                "South Eastern Lowlands",
                "South Pole",
                "South Western Lowlands",
                "Western Lowlands",
            ],
            Self::Eve => &[
                "Akatsuki Lake",
                "Crater Lake",
                "Craters",
                "Eastern Sea",
                "Explodium Sea",
                "Foothills",
                "Highlands",
                "Impact Ejecta",
                "Lowlands",
                "Midlands",
                "Olympus",
                "Peaks",
                "Poles",
                "Shallows",
                "Western Sea",
            ],
            Self::Gilly => &["Highlands", "Lowlands", "Midlands"],
            Self::Kerbin => &[
                "Badlands",
                "Deserts",
                "Grasslands",
                "Highlands",
                "Ice Caps",
                "Mountains",
                "Northern Ice Shelf",
                "Shores",
                "Southern Ice Shelf",
                "Tundra",
                "Water",
            ],
            Self::Mun => &[
                "Canyons",
                "East Crater",
                "East Farside Crater",
                "Farside Basin",
                "Farside Crater",
                "Highland Craters",
                "Highlands",
                "Lowlands",
                "Midland Craters",
                "Midlands",
                "Northern Basin",
                "Northwest Crater",
                "Polar Crater",
                "Polar Lowlands",
                "Poles",
                "Southwest Crater",
                "Twin Craters",
            ],
            Self::Minmus => &[
                "Flats",
                "Great Flats",
                "Greater Flats",
                "Highlands",
                "Lesser Flats",
                "Lowlands",
                "Midlands",
                "Poles",
                "Slopes",
            ],
            Self::Duna => &[
                "Craters",
                "Eastern Canyon",
                "Highlands",
                "Lowlands",
                "Midland Canyon",
                "Midland Sea",
                "Midlands",
                "Northern Basin",
                "Northern Shelf",
                "Polar Craters",
                "Polar Highlands",
                "Poles",
                "Southern Basin",
                "Western Canyon",
            ],
            Self::Ike => &[
                "Central Mountain Range",
                "Eastern Mountain Ridge",
                "Lowlands",
                "Midlands",
                "Polar Lowlands",
                "South Eastern Mountain Range",
                "South Pole",
                "Western Mountain Ridge",
            ],
            Self::Dres => &[
                "Canyons",
                "Highlands",
                "Impact Craters",
                "Impact Ejecta",
                "Lowlands",
                "Midlands",
                "Poles",
                "Ridges",
            ],
            Self::Laythe => &[
                "Crater Bay",
                "Crater Island",
                "Crescent Bay",
                "DeGrasse Sea",
                "Dunes",
                "Peaks",
                "Poles",
                "Shallows",
                "Shores",
                "The Sagen Sea",
            ],
            Self::Vall => &[
                "Highlands",
                "Lowlands",
                "Midlands",
                "Mountains",
                "Northeast Basin",
                "Northwest Basin",
                "Poles",
                "Southern Basin",
                "Southern Valleys",
            ],
            Self::Tylo => &[
                "Gagarin Crater",
                "Galileio Crater",
                "Grissom Crater",
                "Highlands",
                "Lowlands",
                "Mara",
                "Midlands",
                "Minor Craters",
                "Tycho Crater",
            ],
            Self::Bop => &["Peaks", "Poles", "Ridges", "Slopes", "Valley"],
            Self::Pol => &["Highlands", "Lowlands", "Midlands", "Poles"],
            Self::Eeloo => &[
                "Babbage Patch",
                "Craters",
                "Fragipan",
                "Highlands",
                "Ice Canyons",
                "Lowlands",
                "Midlands",
                "Mu Glacier",
                "Northern Glaciers",
                "Poles",
                "Southern Glaciers",
            ],
        }
    }

    /// The multipliers of landed, splashed, flying low, flying high, space low and space
    /// high, from the script's `MULT`. `None` means that the count has no subject in that
    /// situation at this body.
    #[must_use]
    pub fn multipliers(self) -> [Option<Multiplier>; 6] {
        match self {
            Self::Sun => [None, None, m(10), m(10), m(110), m(20)],
            Self::Moho => [m(100), None, None, None, m(80), m(70)],
            Self::Eve => [m(80), m(80), m(60), m(60), m(70), m(50)],
            Self::Gilly => [m(90), None, None, None, m(80), m(60)],
            Self::Kerbin => [m(3), m(4), m(7), m(9), m(10), m(15)],
            Self::Mun => [m(40), None, None, None, m(30), m(20)],
            Self::Minmus => [m(50), None, None, None, m(40), m(25)],
            Self::Duna => [m(80), None, m(50), m(50), m(70), m(50)],
            Self::Ike => [m(80), None, None, None, m(70), m(50)],
            Self::Dres => [m(80), None, None, None, m(70), m(60)],
            Self::Jool => [None, None, m(120), m(90), m(70), m(60)],
            Self::Laythe => [m(140), m(120), m(110), m(100), m(90), m(80)],
            Self::Vall => [m(120), None, None, None, m(90), m(80)],
            Self::Tylo => [m(120), None, None, None, m(100), m(80)],
            Self::Bop => [m(120), None, None, None, m(90), m(80)],
            Self::Pol => [m(120), None, None, None, m(90), m(80)],
            Self::Eeloo => [m(150), None, None, None, m(120), m(100)],
        }
    }

    /// The multiplier of one situation at this body.
    #[must_use]
    pub fn multiplier(self, situation: Situation) -> Option<Multiplier> {
        self.multipliers()[situation.index()]
    }

    /// Looks a body up by the name the game writes.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.name() == name)
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// An experiment the count covers, apart from the surface features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Experiment {
    /// `crewReport`.
    CrewReport,
    /// `evaReport`.
    EvaReport,
    /// `mysteryGoo`.
    MysteryGoo,
    /// `surfaceSample`.
    SurfaceSample,
    /// `mobileMaterialsLab`.
    MobileMaterialsLab,
    /// `temperatureScan`.
    TemperatureScan,
    /// `barometerScan`.
    BarometerScan,
    /// `seismicScan`.
    SeismicScan,
    /// `gravityScan`.
    GravityScan,
    /// `atmosphereAnalysis`.
    AtmosphereAnalysis,
    /// `infraredTelescope`.
    InfraredTelescope,
    /// `magnetometer`.
    Magnetometer,
    /// `evaScience`.
    EvaScience,
}

impl Experiment {
    /// Every experiment, in the order of the script's `EXPS` table.
    pub const ALL: [Self; 13] = [
        Self::CrewReport,
        Self::EvaReport,
        Self::MysteryGoo,
        Self::SurfaceSample,
        Self::MobileMaterialsLab,
        Self::TemperatureScan,
        Self::BarometerScan,
        Self::SeismicScan,
        Self::GravityScan,
        Self::AtmosphereAnalysis,
        Self::InfraredTelescope,
        Self::Magnetometer,
        Self::EvaScience,
    ];

    /// The experiment's identifier, as the game writes it before the `@`.
    #[must_use]
    pub fn id(self) -> &'static str {
        self.row().0
    }

    /// The experiment's science cap before any multiplier.
    #[must_use]
    pub fn cap(self) -> u16 {
        self.row().1
    }

    /// The situations in which the experiment yields science, as the game's mask.
    #[must_use]
    pub fn situation_mask(self) -> u8 {
        self.row().2
    }

    /// The situations in which the experiment's subject depends on the biome.
    #[must_use]
    pub fn biome_mask(self) -> u8 {
        self.row().3
    }

    /// Whether the experiment needs an atmosphere.
    #[must_use]
    pub fn requires_atmosphere(self) -> bool {
        self.row().4
    }

    /// Whether the experiment yields science in this situation.
    #[must_use]
    pub fn allows(self, situation: Situation) -> bool {
        self.situation_mask() & situation.bit() != 0
    }

    /// Whether the experiment's subject in this situation depends on the biome.
    #[must_use]
    pub fn biome_dependent(self, situation: Situation) -> bool {
        self.biome_mask() & situation.bit() != 0
    }

    fn row(self) -> (&'static str, u16, u8, u8, bool) {
        match self {
            Self::CrewReport => ("crewReport", 5, 63, 7, false),
            Self::EvaReport => ("evaReport", 8, 63, 23, false),
            Self::MysteryGoo => ("mysteryGoo", 13, 63, 3, false),
            Self::SurfaceSample => ("surfaceSample", 40, 3, 3, false),
            Self::MobileMaterialsLab => ("mobileMaterialsLab", 32, 63, 3, false),
            Self::TemperatureScan => ("temperatureScan", 8, 63, 7, false),
            Self::BarometerScan => ("barometerScan", 12, 63, 3, false),
            Self::SeismicScan => ("seismicScan", 22, 1, 1, false),
            Self::GravityScan => ("gravityScan", 22, 51, 51, false),
            Self::AtmosphereAnalysis => ("atmosphereAnalysis", 24, 13, 13, true),
            Self::InfraredTelescope => ("infraredTelescope", 22, 32, 0, false),
            Self::Magnetometer => ("magnetometer", 45, 48, 0, false),
            Self::EvaScience => ("evaScience", 25, 63, 0, false),
        }
    }
}

impl fmt::Display for Experiment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

/// Where a biome's name comes from, and so how far it can be trusted to match a save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provenance {
    /// A body biome from the inventory that Phase 0 read from the running game.
    Inventory,
    /// A location biome whose name appears in the subject identifiers of a real save.
    /// The name is a fact.
    ObservedInSave,
    /// A location biome whose name is a display name in the game's localization
    /// dictionary, in the block that holds the observed location names. That the
    /// identifier carries it with spaces removed is an inference from the observed names,
    /// all twelve of which follow that rule.
    Localization,
    /// A location the count includes whose name was not found. Its identifier holds
    /// angle brackets, which no biome name is known to contain, so it never matches a
    /// save.
    Placeholder,
}

/// A Space Center location biome, one of the small biomes of the space centre's
/// buildings and grounds on Kerbin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocationBiome {
    /// The name, with spaces where the source has them.
    pub name: &'static str,
    /// Where the name comes from.
    pub provenance: Provenance,
}

const fn observed(name: &'static str) -> LocationBiome {
    LocationBiome {
        name,
        provenance: Provenance::ObservedInSave,
    }
}

const fn localized(name: &'static str) -> LocationBiome {
    LocationBiome {
        name,
        provenance: Provenance::Localization,
    }
}

const fn placeholder(name: &'static str) -> LocationBiome {
    LocationBiome {
        name,
        provenance: Provenance::Placeholder,
    }
}

/// The 37 location biomes of the count.
///
/// Phase 0 counted 37 from the wiki by number only. Twelve names were observed in the
/// subject identifiers of the operator's older career save on 2026-11-18. Twenty-one more
/// are display names from the same block of the game's localization dictionary,
/// `#autoLOC_6002105` to `#autoLOC_6002147`, that name a part of a building group whose
/// main name was observed. Four remain unnamed, because the dictionary block holds eight
/// further candidates, `Pad`, `Pad Flag Pole`, `Pad Water Tower`, `Pad Round Tank`,
/// `Pad Tanks`, `Launchpad Platform`, `Baikerbanur` and `Baikerbanur LaunchPad`, and
/// nothing read offline shows which of them the wiki's 37 include. The wiki refused an
/// automated read on 2026-11-18.
pub const LOCATION_BIOMES: [LocationBiome; 37] = [
    observed("KSC"),
    observed("Administration"),
    observed("AstronautComplex"),
    observed("Crawlerway"),
    observed("FlagPole"),
    observed("LaunchPad"),
    observed("MissionControl"),
    observed("R&D"),
    observed("Runway"),
    observed("SPH"),
    observed("TrackingStation"),
    observed("VAB"),
    localized("R&D Central Building"),
    localized("R&D Corner Lab"),
    localized("R&D Main Building"),
    localized("R&D Observatory"),
    localized("R&D Side Lab"),
    localized("R&D Small Lab"),
    localized("R&D Tanks"),
    localized("R&D Wind Tunnel"),
    localized("SPH Main Building"),
    localized("SPH Round Tank"),
    localized("SPH Tanks"),
    localized("SPH Water Tower"),
    localized("Tracking Station Dish East"),
    localized("Tracking Station Dish North"),
    localized("Tracking Station Dish South"),
    localized("Tracking Station Hub"),
    localized("VAB Main Building"),
    localized("VAB Pod Memorial"),
    localized("VAB Round Tank"),
    localized("VAB South Complex"),
    localized("VAB Tanks"),
    placeholder("<unnamed location 1>"),
    placeholder("<unnamed location 2>"),
    placeholder("<unnamed location 3>"),
    placeholder("<unnamed location 4>"),
];

/// A Breaking Ground surface feature, whose scan is one subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceFeature {
    /// The experiment identifier, from `ScienceDefs.cfg`.
    pub id: &'static str,
    /// The body the feature stands on. This is an inference from the identifier, which
    /// names the body, and `DunaEjectaOnIke` names Ike as the place.
    pub body: Body,
    /// The science cap, from `ScienceDefs.cfg`.
    pub cap: u16,
}

const fn feature(id: &'static str, body: Body, cap: u16) -> SurfaceFeature {
    SurfaceFeature { id, body, cap }
}

/// The 34 surface features, in the order of Breaking Ground's `ScienceDefs.cfg`. Each has
/// a situation mask of 1 and a biome mask of 0 there, so its subject is landed and has no
/// biome. Twelve have a cap of 30, eleven of 45 and eleven of 60, which sums to 1,515.
/// The Phase 0 script's split, twelve, twelve and ten, sums to 1,500, so it undercounts
/// the features by 15 science before the difficulty multiplier.
pub const SURFACE_FEATURES: [SurfaceFeature; 34] = [
    feature("ROCScience_KerbinBaobabTree", Body::Kerbin, 60),
    feature("ROCScience_KerbinGiantQuartz", Body::Kerbin, 45),
    feature("ROCScience_GillyRidgeline", Body::Gilly, 45),
    feature("ROCScience_DunaEjectaOnIke", Body::Ike, 30),
    feature("ROCScience_BopGravelPile", Body::Bop, 45),
    feature("ROCScience_PolYellowStones", Body::Pol, 30),
    feature("ROCScience_MinmusGreenSandstone", Body::Minmus, 30),
    feature("ROCScience_MinmusOlivineFormation", Body::Minmus, 60),
    feature("ROCScience_MohoStone", Body::Moho, 30),
    feature("ROCScience_MohoWrinkleRidge", Body::Moho, 60),
    feature("ROCScience_EveVolcanicRock", Body::Eve, 30),
    feature("ROCScience_EveBasaltFormation", Body::Eve, 45),
    feature("ROCScience_EvePancakeDome", Body::Eve, 60),
    feature("ROCScience_DunaStone", Body::Duna, 30),
    feature("ROCScience_DunaMeteorite", Body::Duna, 45),
    feature("ROCScience_DunaDune", Body::Duna, 60),
    feature("ROCScience_DunaBlueberries", Body::Duna, 30),
    feature("ROCScience_DresMeteorite", Body::Dres, 45),
    feature("ROCScience_DresCrater", Body::Dres, 60),
    feature("ROCScience_MunStone", Body::Mun, 30),
    feature("ROCScience_MunCrater", Body::Mun, 45),
    feature("ROCScience_MunLargeCrater", Body::Mun, 60),
    feature("ROCScience_LaytheStone", Body::Laythe, 30),
    feature("ROCScience_LaytheBoulder", Body::Laythe, 45),
    feature("ROCScience_LaytheGeyser", Body::Laythe, 60),
    feature("ROCScience_VallIceChunk", Body::Vall, 30),
    feature("ROCScience_VallStone", Body::Vall, 45),
    feature("ROCScience_VallCryovolcano", Body::Vall, 60),
    feature("ROCScience_TyloLightStone", Body::Tylo, 30),
    feature("ROCScience_TyloDarkBoulder", Body::Tylo, 45),
    feature("ROCScience_TyloCheckerboard", Body::Tylo, 60),
    feature("ROCScience_EelooIceChunk", Body::Eeloo, 30),
    feature("ROCScience_EelooBrownBoulder", Body::Eeloo, 45),
    feature("ROCScience_EelooBerg", Body::Eeloo, 60),
];

/// The Hard difficulty's science multiplier, 0.6, in tenths.
pub const HARD_TENTHS: u16 = 6;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn situation_bits_are_distinct_powers_of_two_in_order() {
        let mut expected = 1_u8;
        for situation in Situation::ALL {
            assert_eq!(situation.bit(), expected);
            expected = expected.wrapping_shl(1);
        }
    }

    #[test]
    fn multiplier_refuses_zero() {
        assert_eq!(Multiplier::from_tenths(0), None);
        assert_eq!(Multiplier::from_tenths(3).map(Multiplier::get), Some(0.3));
    }

    #[test]
    fn body_names_round_trip_and_unknown_names_are_refused() {
        for body in Body::ALL {
            assert_eq!(Body::from_name(body.name()), Some(body));
        }
        assert_eq!(Body::from_name("kerbin"), None);
        assert_eq!(Body::from_name(""), None);
    }

    #[test]
    fn location_names_are_distinct() {
        for (i, a) in LOCATION_BIOMES.iter().enumerate() {
            for b in LOCATION_BIOMES.iter().skip(i + 1) {
                assert_ne!(a.name.replace(' ', ""), b.name.replace(' ', ""));
            }
        }
    }

    #[test]
    fn surface_feature_caps_split_twelve_eleven_eleven() {
        let count = |cap| SURFACE_FEATURES.iter().filter(|f| f.cap == cap).count();
        assert_eq!((count(30), count(45), count(60)), (12, 11, 11));
        assert!(
            SURFACE_FEATURES
                .iter()
                .all(|f| f.id.starts_with("ROCScience_"))
        );
    }
}
