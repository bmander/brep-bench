//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, grown by 1 with its edges kept sharp: each face moved 1 outward, the faces extended
//! to meet.
//!
//! Refused. cad-kernel has no offset of a solid: neither growing nor shrinking one is in its API
//! (its README lists "shelling and offsets" among what comes next).
//!
//! Solvent's `std.rounds.Grown` rounds every convex edge; there is no offset that keeps the edges
//! sharp.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no offset of a solid: cad-kernel cannot grow a body, with its edges rounded or kept sharp".into())
}
