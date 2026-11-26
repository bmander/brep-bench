//! The block from (0, 0, 0) to (20, 20, 6) with a square pocket, from (6, 6) to (14, 14), cut
//! down from its top to a flat floor at z = 3, grown by 1: the solid of every point within 1 of
//! it. The pocket's floor rises to z = 4 and its walls come in 1, its concave edges sharp; its
//! rim rounds in quarter cylinders.
//!
//! The block a rectangle extruded, the pocket a square extruded from the floor up past the top
//! and cut; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, p3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(20.0, 20.0, 6.0))?;
    let pocket = block(k, p3(6.0, 6.0, 3.0), p3(14.0, 14.0, 7.0))?;
    let pocketed = cut(k, b, pocket)?;
    grown(k, pocketed, 1.0)
}
