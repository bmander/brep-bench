//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), extruded 8
//! along +z, shrunk by 1: its concave edge rounded, its convex edges kept sharp.
//!
//! Refused. cad-kernel has no offset of a solid: neither growing nor shrinking one is in its API
//! (its README lists "shelling and offsets" among what comes next).
//!
//! Solvent's `std.rounds.Grown` with a distance below nothing reads a body shrunk point by point,
//! and an exact export refuses it.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no offset of a solid: cad-kernel cannot shrink a body, and Solvent's std.rounds.Grown reads a body shrunk only point by point".into())
}
