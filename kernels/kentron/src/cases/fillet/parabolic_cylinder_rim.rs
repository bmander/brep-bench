//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top blended by a conic of rho 0.5, a parabola tangent to the top and to the side each 2 from
//! the edge, turned round the axis.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // (A point on the rim away from the side's seam.)
    let rim = edge_through(m, cylinder, p(0.0, 5.0, 10.0))?;
    Ok(blend::fillet_conic(m, cylinder, &[rim], 2.0, 0.5)?)
}
