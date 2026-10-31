//! A square spire from z = 0 to its apex (0, 0, 24): its section at every height a square
//! centred on the z axis, its sides parallel to the x and y axes, with its corners on four
//! ridges, the cubic Bézier curve through `RIDGE` and its mirror images in the xz plane, the yz
//! plane and both. It swells from 18 × 18 at the foot to 24 × 24 and closes to the apex.
//!
//! arris's `loft` takes no guides, and no section of nothing: the spire is assembled from its
//! faces by `solids::rectangle_on_guides`, each wall on the extrusion of the ridge seen square to
//! it, the four walls meeting at the apex.
use crate::solids::rectangle_on_guides;
use crate::Refusal;
use arris::topo::{Body, Model};

/// The ridge through the corner with x and y positive, in the plane x = y.
const RIDGE: [[f64; 3]; 4] =
    [[9.0, 9.0, 0.0], [15.0, 15.0, 8.0], [12.0, 12.0, 19.0], [0.0, 0.0, 24.0]];

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    rectangle_on_guides(m, RIDGE)
}
