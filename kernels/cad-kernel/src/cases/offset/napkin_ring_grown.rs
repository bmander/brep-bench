//! A ball of radius 5 centred on the origin, with a hole of radius 3 bored through it along the
//! z axis, grown by 1.
//!
//! Refused. cad-kernel has no offset of a solid: neither growing nor shrinking one is in its API
//! (its README lists "shelling and offsets" among what comes next).
//!
//! Solvent's `std.rounds.Grown` does not build this one exactly either. It builds a body grown
//! only where its faces are planes and cylinders meeting at a prism's corners, at concave planar
//! edges or at a boss's foot. Anywhere else it reads the body point by point, and an exact export
//! refuses it.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no offset of a solid: cad-kernel cannot grow a body, and Solvent's std.rounds.Grown builds one exactly only where its faces are planes and cylinders meeting at a prism's corners, at concave planar edges or at a boss's foot".into())
}
