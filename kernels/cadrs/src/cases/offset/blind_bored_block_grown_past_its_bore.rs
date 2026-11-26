//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
//! the top to a flat floor at z = 2, grown by 4, more than the bore's radius: the solid of every
//! point within 4 of it. The bore fills; over its mouth the grown top dips in a dimple, the inner
//! part of a spindle torus about the rim, down to a point on the axis √7 above the top.
//!
//! The block a rectangle extruded, the bore a circle extruded from its floor up past the top and
//! cut; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let bore = cylinder(k, p3(0.0, 0.0, 2.0), v3(0.0, 0.0, 1.0), 3.0, 10.0)?;
    let bored = cut(k, b, bore)?;
    grown(k, bored, 4.0)
}
