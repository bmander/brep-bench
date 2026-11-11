//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded to radius 2.
//!
//! The cube is a square extruded; the edge is picked by the point at its middle and filleted,
//! radius 2.
use crate::bench::{block, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let edge = k.edge_at(cube, p3(10.0, 10.0, 5.0))?;
    fillet(k, cube, &[edge], 2.0)
}
