//! A hub, the tube about the z axis from z = 0 to z = 20 of outer radius 10 and bore of radius 4,
//! with a keyway cut along the bore's whole length, the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away: its
//! floor y = −5 and its walls x = −1 and x = 1; the keyway's three faces then deleted, the bore
//! closing over it and the hub's ends growing back to whole rings.
//!
//! The keyed hub is a `prism` of a `disc` of radius 10 cut by a `prism` of a `disc` of radius 4 and
//! a box, a `prism` of a rectangle, both running past both ends; the keyway's floor and walls are
//! picked by a point on each and deleted together (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const UP: V = [0., 0., 1.];

pub fn build() -> Result<Brep, String> {
    let outer = prism(&disc([0., 0., 0.], UP, 10.), 0., 20.)?;
    let inner = prism(&disc([0., 0., -1.], UP, 4.), 0., 22.)?;
    let tube = combined(&outer, &inner, Op::Cut, 0.)?;
    let keyed = combined(&tube, &block([-1., -5., -1.], [1., 0., 21.])?, Op::Cut, 0.)?;
    // the walls run from the bore, at y = −√15, down to the floor
    deleted_through(&keyed, &[[0., -5., 10.], [-1., -4.5, 10.], [1., -4.5, 10.]])
}
