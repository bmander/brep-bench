//! A cylinder of radius 5 from z = 0 to 10 capped by a hemisphere, shelled inward 1 thick, open at
//! its dome, which is tangent to its side.
//!
//! Refused. cad-kernel has no shell: hollowing a solid to a wall, open or closed, is not in its
//! API (its README lists "shelling and offsets" among what comes next), and it has no offset of a
//! face or a solid to make the wall's inside from.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no shell: cad-kernel cannot hollow a body to a wall, and has no offset of a face or a solid to make its inside from".into())
}
