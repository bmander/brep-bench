//! A plate from the origin to (40, 30, 6) with all twelve of its edges rounded to radius 3, half
//! its thickness: the side faces vanish, the top and bottom rounds of each side meeting along its
//! middle, and each corner is a quarter of one ball.
//!
//! The plate is a rectangle extruded; its twelve edges are rounded in one Fillet feature.
use crate::bench::{block, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(0.0, 0.0, 0.0), p3(40.0, 30.0, 6.0))?;
    let edges = k.edges_but_seams(plate)?;
    fillet(k, plate, &edges, 3.0)
}
