//! The lower half of the ball of radius 5 centred on the origin, shelled inward 1 thick, open at
//! its flat face: a bowl.
//!
//! Refused. cad-kernel has no shell: hollowing a solid to a wall, open or closed, is not in its
//! API (its README lists "shelling and offsets" among what comes next), and it has no offset of a
//! face or a solid to make the wall's inside from.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no shell: cad-kernel cannot hollow a body to a wall, and has no offset of a face or a solid to make its inside from".into())
}
