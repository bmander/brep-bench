//! The plus-shaped plate of `offset/plus-shaped-plate-grown`, extruded 4 along +z, shrunk by 1:
//! the solid of every point at least 1 from its outside. Each arm is left 1 wide; the four
//! concave vertical edges are rounded to radius 1 by the shrink.
//!
//! The outline extruded, then cadrs's offset inward.
use crate::bench::extrude;
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline = Loop::polygon(&super::r#plus_shaped_plate_grown::PLUS);
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 4.0)?;
    shrunk(k, plate, 1.0)
}
