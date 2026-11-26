//! The box from the origin to (20, 10, 10), the edge where its top meets its end x = 20 blended by
//! a conic of rho 0.5, a parabola tangent to the top and to the end each 3 from the edge.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, block, p(20.0, 5.0, 10.0))?;
    Ok(blend::fillet_conic(m, block, &[edge], 3.0, 0.5)?)
}
