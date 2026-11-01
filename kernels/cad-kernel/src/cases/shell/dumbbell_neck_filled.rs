//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, extruded 6,
//! shelled inward 1.5 thick with its top open: the neck fills, and each pocket bulges towards it in
//! arcs about the neck's corners.
//!
//! Refused. cad-kernel has no shell: hollowing a solid to a wall, open or closed, is not in its
//! API (its README lists "shelling and offsets" among what comes next), and it has no offset of a
//! face or a solid to make the wall's inside from.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no shell: cad-kernel cannot hollow a body to a wall, and has no offset of a face or a solid to make its inside from".into())
}
