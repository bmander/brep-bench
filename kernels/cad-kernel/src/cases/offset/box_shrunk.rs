//! The box from the origin to (10, 8, 6), shrunk by 1: every point at least 1 from its outside.
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
