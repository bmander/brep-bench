//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of two
//! overlapping beads on one axis along y slid 40 along z past it: the first bead's centre at
//! (11.5, 0, −9), ring 6 and tube 2; the second's at (11.5, 1.5, −9), ring 6 and tube 1.5.
//!
//! solidcraft-kernel has no sweep of a solid, so the wheel's sweep is built by elementary
//! geometry. The sweep of a union is the union of its parts' sweeps, and each bead's is, as in
//! `torus-slid-through-rod`, its shadow along z (a slot within its tube's radius of the segment
//! its tube's centres cast) extruded from its centre's height where it starts to where it ends,
//! with the bead at each end. Those end beads lie wholly below z = −1 and above z = 21, clear of
//! the rod, so the two slots' prisms are what the sweep takes from the rod.
use super::torus_slid_through_rod::shadow;
use crate::solids::*;

pub fn build() -> R<Body> {
    let mut rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    for (y, tube) in [(0.0, 2.0), (1.5, 1.5)] {
        let (plane, slot) = shadow(11.5, y, -9.0, 6.0, tube);
        rod = cut(&rod, &extrude(&plane, slot, 0.0, 40.0)?)?;
    }
    Ok(rod)
}
