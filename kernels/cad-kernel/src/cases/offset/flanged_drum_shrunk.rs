//! A drum of radius 8 from z = 0 to z = 6, with a flange 1.6 thick out to radius 11 round its
//! side from z = 2.2 to z = 3.8, shrunk by 1, so that the flange is gone.
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
