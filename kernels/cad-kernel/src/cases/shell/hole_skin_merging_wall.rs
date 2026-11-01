//! The block from the origin to (20, 20, 10) bored through along x = 15.5, y = 10 with a hole of
//! radius 3, shelled inward 1 thick with its top open: the bore's tube runs into the side wall.
//!
//! Refused. cad-kernel has no shell: hollowing a solid to a wall, open or closed, is not in its
//! API (its README lists "shelling and offsets" among what comes next), and it has no offset of a
//! face or a solid to make the wall's inside from.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no shell: cad-kernel cannot hollow a body to a wall, and has no offset of a face or a solid to make its inside from".into())
}
