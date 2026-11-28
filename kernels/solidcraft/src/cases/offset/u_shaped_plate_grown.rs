//! The U-shaped plate, its outline the polygon through (0, 0), (9, 0), (9, 9), (6, 9), (6, 3),
//! (3, 3), (3, 9) and (0, 9) extruded 2 along +z, grown by 1: every point within 1 of it, its
//! convex edges rounded and the slot's two concave ones sharp, the kernel's offset with its convex
//! edges rounded.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [9.0, 0.0, 0.0],
        [9.0, 9.0, 0.0],
        [6.0, 9.0, 0.0],
        [6.0, 3.0, 0.0],
        [3.0, 3.0, 0.0],
        [3.0, 9.0, 0.0],
        [0.0, 9.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 2.0, 0.0, DraftCorner::default(), true)?;
    offset_rounded(&plate, 1.0)
}
