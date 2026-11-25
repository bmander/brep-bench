//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway cut
//! into it, the box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away, running from z = 8 out through the top: its
//! floor y = −3, its walls x = −1 and x = 1 and its end z = 8; all four then deleted, the cylinder
//! closing over the keyway and the top growing back to a whole disc.
//!
//! The keyed shaft is a `prism` of a `disc` cut by a box, a `prism` of a rectangle, from z = 8 up
//! past the top; the keyway's floor, walls and end are picked by a point on each and deleted
//! together (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let shaft = prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 20.)?;
    let keyed = combined(&shaft, &block([-1., -10., 8.], [1., -3., 21.])?, Op::Cut, 0.)?;
    // the walls run from the floor down to the cylinder, at y = −√24
    deleted_through(&keyed, &[[0., -3., 12.], [-1., -4., 12.], [1., -4., 12.], [0., -4., 8.]])
}
