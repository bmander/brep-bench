//! A solid torus about the z axis, its tube of radius 3 about a middle circle of radius 10,
//! shrunk by 1.5.
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
