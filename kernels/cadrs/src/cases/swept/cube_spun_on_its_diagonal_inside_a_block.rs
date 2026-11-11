//! A block (the box from (−3, −3, −3) to (3, 3, 3)) with a sealed void: everything the cube from
//! (−1, −1, −1) to (1, 1, 1) passes through as it turns a whole turn about its long diagonal.
//!
//! What the cube passes through is cadrs's sweep of a solid (one whole turn about the line through
//! (−1, −1, −1) and (1, 1, 1)), cut from the block.
use crate::bench::{block, cut, p3, v3};
use crate::shared::swept::swung;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-3.0, -3.0, -3.0), p3(3.0, 3.0, 3.0))?;
    let cube = block(k, p3(-1.0, -1.0, -1.0), p3(1.0, 1.0, 1.0))?;
    let swept = swung(k, cube, Axis::new(p3(-1.0, -1.0, -1.0), v3(1.0, 1.0, 1.0)), 0.0, 360.0)?;
    cut(k, b, swept)
}
