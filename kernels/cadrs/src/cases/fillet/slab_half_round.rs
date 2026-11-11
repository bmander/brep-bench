//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x (at
//! y = 0 and y = 4) rounded to radius 2, half the top's width: the rounds are quarters of the
//! same cylinder of radius 2 on the line y = 2, z = 4, the top vanishes and the block's top is a
//! half-cylinder.
//!
//! The block is a rectangle extruded; both edges are picked by a point on each and rounded in
//! one Fillet feature, radius 2.
use crate::bench::{block, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 4.0, 6.0))?;
    let edges = k.edges_at(b, &[p3(5.0, 0.0, 6.0), p3(5.0, 4.0, 6.0)])?;
    fillet(k, b, &edges, 2.0)
}
