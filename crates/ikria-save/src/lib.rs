//! A read-only reader for Kerbal Space Program 1 save files.
//!
//! A save, a `.sfs` file, is a document in the game's ConfigNode text format whose single
//! top-level node is `GAME`. [`node`] parses the format into a tree, and [`Save`] holds
//! the `GAME` node and extracts typed records from it, namely the agency's totals, facility
//! levels, researched technologies, science subjects, the crew roster, vessels, and
//! contracts. The design is task T8 of `docs/roadmap/V0_1_0.md`, and the science ledger
//! of task T9 builds on it.
//!
//! # Trust
//!
//! A save is untrusted input. The reader never writes, bounds the size and nesting depth
//! of what it reads, uses no recursion while parsing, and returns every refusal as a
//! typed error that names a line. No input is known to make it panic. That claim rests
//! on the tests and on review, not on a proof or on fuzzing.
//!
//! # Extraction
//!
//! Unknown fields and unknown child nodes are ignored, so that a save with fields from
//! another game version or a modification still reads. A required field that is missing,
//! or present with a value of the wrong form, is an error, and the extractor stops at
//! the first one. Where the save repeats a node or key that is expected once, the first
//! is used. Facts and inferences about the format are marked in each record's
//! documentation and in the crate's `README.md`.

#![forbid(unsafe_code)]

pub mod career;
pub mod contracts;
mod field;
pub mod node;
pub mod roster;
pub mod vessels;

use core::fmt;
use std::io::Read;
use std::path::Path;

pub use career::{
    Agency, Facility, FacilityLevel, FacilityRecord, ScienceSubject, TechState, Technology,
};
pub use contracts::{Contract, ContractList, ContractState, ContractValues};
pub use field::{ExtractError, Finite, NonNegative};
pub use node::{Node, ParseError, ParseLimits, Value};
pub use roster::{Kerbal, KerbalType, RosterStatus, Trait};
pub use vessels::{BodyIndex, Vessel, VesselSituation, VesselType};

/// Why a save could not be read.
#[derive(Debug)]
pub enum SaveError {
    /// The file could not be opened or read.
    Io(std::io::Error),
    /// The text is not a well-formed document within the limits.
    Parse(ParseError),
    /// The document has no `GAME` node.
    Extract(ExtractError),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "reading the save: {e}"),
            Self::Parse(e) => write!(f, "parsing the save: {e}"),
            Self::Extract(e) => write!(f, "reading the save's structure: {e}"),
        }
    }
}

impl std::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Parse(e) => Some(e),
            Self::Extract(e) => Some(e),
        }
    }
}

impl From<ParseError> for SaveError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<ExtractError> for SaveError {
    fn from(e: ExtractError) -> Self {
        Self::Extract(e)
    }
}

/// A parsed save, holding its `GAME` node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Save {
    game: Node,
}

impl Save {
    /// Parses a save's text under the default limits.
    ///
    /// # Errors
    ///
    /// A [`ParseError`], or an [`ExtractError`] when there is no `GAME` node.
    pub fn parse(text: &str) -> Result<Self, SaveError> {
        Self::parse_with_limits(text, ParseLimits::DEFAULT)
    }

    /// Parses a save's text under the given limits.
    ///
    /// # Errors
    ///
    /// A [`ParseError`], or an [`ExtractError`] when there is no `GAME` node.
    pub fn parse_with_limits(text: &str, limits: ParseLimits) -> Result<Self, SaveError> {
        Self::from_root(node::parse_with_limits(text, limits)?)
    }

    /// Reads and parses a save file under the default limits. The file is opened for
    /// reading only.
    ///
    /// # Errors
    ///
    /// As [`Save::read_with_limits`].
    pub fn read(path: &Path) -> Result<Self, SaveError> {
        Self::read_with_limits(path, ParseLimits::DEFAULT)
    }

    /// Reads and parses a save file under the given limits. The file is opened for
    /// reading only, and reading stops one byte past the size limit, so a file that
    /// grows while it is read is still bounded.
    ///
    /// # Errors
    ///
    /// An I/O error, a [`ParseError`] for a file over the limit, not UTF-8 or not well
    /// formed, or an [`ExtractError`] when there is no `GAME` node.
    pub fn read_with_limits(path: &Path, limits: ParseLimits) -> Result<Self, SaveError> {
        let file = std::fs::File::open(path).map_err(SaveError::Io)?;
        let cap = u64::try_from(limits.max_bytes())
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let mut bytes = Vec::new();
        file.take(cap)
            .read_to_end(&mut bytes)
            .map_err(SaveError::Io)?;
        Self::from_root(node::parse_bytes(&bytes, limits)?)
    }

    fn from_root(mut root: Node) -> Result<Self, SaveError> {
        match root.take_child("GAME") {
            Some(game) => Ok(Self { game }),
            None => Err(field::missing_node("GAME", &root).into()),
        }
    }

    /// The `GAME` node, for fields this crate does not extract.
    #[must_use]
    pub fn game(&self) -> &Node {
        &self.game
    }

    /// The first `SCENARIO` node with this `name`.
    ///
    /// # Errors
    ///
    /// [`ExtractError::MissingNode`] when there is none.
    pub fn scenario(&self, name: &str) -> Result<&Node, ExtractError> {
        career::scenario(&self.game, name)
    }

    /// The agency's funds, science and reputation. See [`career::agency`].
    ///
    /// # Errors
    ///
    /// As [`career::agency`].
    pub fn agency(&self) -> Result<Agency, ExtractError> {
        career::agency(&self.game)
    }

    /// The stock facilities' levels. See [`career::facilities`].
    ///
    /// # Errors
    ///
    /// As [`career::facilities`].
    pub fn facilities(&self) -> Result<Vec<FacilityRecord>, ExtractError> {
        career::facilities(&self.game)
    }

    /// The technology nodes and their purchased parts. See [`career::technologies`].
    ///
    /// # Errors
    ///
    /// As [`career::technologies`].
    pub fn technologies(&self) -> Result<Vec<Technology>, ExtractError> {
        career::technologies(&self.game)
    }

    /// The science subjects. See [`career::science_subjects`].
    ///
    /// # Errors
    ///
    /// As [`career::science_subjects`].
    pub fn science_subjects(&self) -> Result<Vec<ScienceSubject>, ExtractError> {
        career::science_subjects(&self.game)
    }

    /// The crew roster. See [`roster::roster`].
    ///
    /// # Errors
    ///
    /// As [`roster::roster`].
    pub fn roster(&self) -> Result<Vec<Kerbal>, ExtractError> {
        roster::roster(&self.game)
    }

    /// The vessels, including kerbals outside a vessel and flags. See
    /// [`vessels::vessels`].
    ///
    /// # Errors
    ///
    /// As [`vessels::vessels`].
    pub fn vessels(&self) -> Result<Vec<Vessel>, ExtractError> {
        vessels::vessels(&self.game)
    }

    /// The contracts. See [`contracts::contracts`].
    ///
    /// # Errors
    ///
    /// As [`contracts::contracts`].
    pub fn contracts(&self) -> Result<Vec<Contract>, ExtractError> {
        contracts::contracts(&self.game)
    }
}
