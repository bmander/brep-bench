//! A cone on the z axis with a base of radius 3 at z = 0 and its apex at (0, 0, 4), grown by 1.
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
