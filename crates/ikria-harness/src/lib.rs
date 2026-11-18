//! The Ikria harness, which runs Keleusma control loops for Kerbal Space Program craft.
//!
//! The design is `docs/architecture/HARNESS_DESIGN.md` in this repository. The single
//! vessel scheduler in [`harness`] steps a control loop against a [`host::Host`]. The
//! scripted host supplies deterministic telemetry without the game. [`control`] adapts
//! the representative Keleusma loop in [`attitude`] for those tests.

// Keleusma's `KeleusmaType` derive expands to paths in `alloc`, so the crate names it.
extern crate alloc;

pub mod attitude;
pub mod control;
pub mod harness;
pub mod host;

pub mod flight;

pub mod session;
