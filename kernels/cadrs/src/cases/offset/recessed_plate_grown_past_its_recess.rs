//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its
//! top about the z axis (a flat floor at z = 3), grown by 4, more than the recess's radius: the
//! solid of every point within 4 of it. The grown top dips over the recess: a flat at z = 7 out
//! to radius 3 − √7, then the inner side of a tube of radius 4 about the rim.
//!
//! The plate a rectangle extruded, the recess a circle extruded from its floor up past the top
//! and cut; then cadrs's offset with rounded edges.
use crate::bench::{block, cut, cylinder, p3, v3};
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-10.0, -10.0, 0.0), p3(10.0, 10.0, 4.0))?;
    let recess = cylinder(k, p3(0.0, 0.0, 3.0), v3(0.0, 0.0, 1.0), 3.0, 2.0)?;
    let recessed = cut(k, plate, recess)?;
    grown(k, recessed, 4.0)
}
