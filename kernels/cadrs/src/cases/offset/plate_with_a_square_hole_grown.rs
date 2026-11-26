//! The square plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7)
//! through it, grown by 1: the solid of every point within 1 of it. The hole's walls move 1 into
//! it and its four vertical edges, concave, stay sharp.
//!
//! The plate a rectangle extruded, the hole a square extruded through it and cut; then cadrs's
//! offset with rounded edges.
use crate::bench::{block, cut, p3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 2.0))?;
    let hole = block(k, p3(3.0, 3.0, -1.0), p3(7.0, 7.0, 3.0))?;
    let holed = cut(k, plate, hole)?;
    grown(k, holed, 1.0)
}
