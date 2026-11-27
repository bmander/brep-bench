//! What the box from the origin to (10, 6, 4) fills as it slides along (8, 5, 0) without turning.
use anvil_brep::Solid;

use crate::shared::{v3, zonotope};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. A box slid along a line fills the zonotope of its three
    // edges and the slide: the box's faces and the parallelograms each of its edges sweeps
    let (box_, slide) = ([v3(10., 0., 0.), v3(0., 6., 0.), v3(0., 0., 4.)], v3(8., 5., 0.));
    let center = v3(5., 3., 2.) + slide * 0.5;
    zonotope(center, &[box_[0], box_[1], box_[2], slide])
}
