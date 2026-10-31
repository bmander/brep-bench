//! A slab from z = 0 to z = 24 whose section at every height is a rectangle centred on the z
//! axis, its sides parallel to the x and y axes, with its corners on four guides: the cubic
//! Bézier curve through `GUIDE` and its mirror images in the xz plane, the yz plane and both.
//! It runs from 24 × 6.4 at the foot, through 30 × 0.4 at its thinnest, to 6 × 24.4 at the top.
//!
//! arris's `loft` takes no guides: the slab is assembled from its faces by
//! `solids::rectangle_on_guides`, each wall on the extrusion of the guide seen square to it.
use crate::solids::rectangle_on_guides;
use crate::Refusal;
use arris::topo::{Body, Model};

/// The guide through the corner with x and y positive, (p, q, z) at each point.
const GUIDE: [[f64; 3]; 4] =
    [[12.0, 3.2, 0.0], [18.0, -2.8, 8.0], [15.0, 0.2, 19.0], [3.0, 12.2, 24.0]];

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    rectangle_on_guides(m, GUIDE)
}
