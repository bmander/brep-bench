//! A barrel: the ball of radius 5 about the origin cut by the planes z = −3 and z = 3, the part
//! between them kept (flat ends, disks of radius 4), shrunk by 1: the solid of every point at
//! least 1 from its outside, the ball of radius 4 cut by z = ±2.
//!
//! The ball is cadrs's sphere primitive, its ends cut flat by a common with a block (a
//! rectangle extruded from z = −3 to 3); then cadrs's offset inward.
use crate::bench::{ball, block, common, p3};
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let slab = block(k, p3(-6.0, -6.0, -3.0), p3(6.0, 6.0, 3.0))?;
    let barrel = common(k, b, slab)?;
    shrunk(k, barrel, 1.0)
}
