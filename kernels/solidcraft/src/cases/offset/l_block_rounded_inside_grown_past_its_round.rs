//! The L-shaped plate of `offset/l-block-grown` (the polygon through (0, 0), (20, 0), (20, 5),
//! (5, 5), (5, 15) and (0, 15), extruded 8 along +z) with its concave edge, along (5, 5), rounded
//! to radius 4, grown by 5, more than the round's radius: every point within 5 of it, the kernel's
//! offset with its convex edges rounded.
use crate::bench::polygon;
use crate::shared::edit::rounded;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [20.0, 0.0, 0.0],
        [20.0, 5.0, 0.0],
        [5.0, 5.0, 0.0],
        [5.0, 15.0, 0.0],
        [0.0, 15.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 8.0, 0.0, DraftCorner::default(), true)?;
    // the inside edge, picked at its middle
    let plate = rounded(&plate, &[[5.0, 5.0, 4.0]], 4.0)?;
    offset_rounded(&plate, 5.0)
}
