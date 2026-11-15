//! The Ikria harness, which runs Keleusma control loops for Kerbal Space Program craft.
//!
//! The design is `docs/architecture/HARNESS_DESIGN.md` in this repository. At this stage
//! the crate holds only the representative loop of Phase 0 experiment 8, in [`attitude`].

// Keleusma's `KeleusmaType` derive expands to paths in `alloc`, so the crate names it.
extern crate alloc;

pub mod attitude;
