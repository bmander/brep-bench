//! The U-shaped plate whose outline is the polygon through (0, 0), (9, 0), (9, 9), (6, 9),
//! (6, 3), (3, 3), (3, 9) and (0, 9), extruded 2 along +z, grown by 1: the solid of every point
//! within 1 of it. The slot narrows to 1 wide; its two concave vertical edges stay sharp.
//!
//! The outline extruded, then cadrs's offset with rounded edges.
use crate::bench::extrude;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline = Loop::polygon(&[
        [0.0, 0.0], [9.0, 0.0], [9.0, 9.0], [6.0, 9.0], [6.0, 3.0], [3.0, 3.0], [3.0, 9.0], [0.0, 9.0],
    ]);
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 2.0)?;
    grown(k, plate, 1.0)
}
