//! The plus-shaped plate, its outline the polygon through (3, 0), (6, 0), (6, 3), (9, 3), (9, 6),
//! (6, 6), (6, 9), (3, 9), (3, 6), (0, 6), (0, 3) and (3, 3) extruded 2 along +z, grown by 1: every
//! point within 1 of it, its convex edges rounded and its four concave ones sharp, the kernel's
//! offset with its convex edges rounded.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

/// The plus outline in the plane z = 0, extruded `thickness` along +z.
pub fn plus_plate(thickness: f64) -> Result<Body, Refusal> {
    let outline = polygon(&[
        [3.0, 0.0, 0.0],
        [6.0, 0.0, 0.0],
        [6.0, 3.0, 0.0],
        [9.0, 3.0, 0.0],
        [9.0, 6.0, 0.0],
        [6.0, 6.0, 0.0],
        [6.0, 9.0, 0.0],
        [3.0, 9.0, 0.0],
        [3.0, 6.0, 0.0],
        [0.0, 6.0, 0.0],
        [0.0, 3.0, 0.0],
        [3.0, 3.0, 0.0],
    ]);
    Ok(extrude(&outline, [0.0, 0.0, 1.0], thickness, 0.0, DraftCorner::default(), true)?)
}

pub fn build() -> Result<Body, Refusal> {
    offset_rounded(&plus_plate(2.0)?, 1.0)
}
