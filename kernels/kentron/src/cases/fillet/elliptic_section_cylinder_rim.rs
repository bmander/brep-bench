//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top rounded unequally, 1 across the top and 3 down the side, its section a quarter ellipse.
use crate::bench::{edge_through, face_through, p};
use crate::Refusal;
use kentron::blend::{self, ConicFilletEdge, ConicSection};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // (A point on the rim away from the side's seam.)
    let rim = edge_through(m, cylinder, p(0.0, 5.0, 10.0))?;
    let top = face_through(m, cylinder, p(0.0, 0.0, 10.0))?;
    let round = ConicFilletEdge { edge: rim, distances: (1.0, 3.0), face: Some(top), section: ConicSection::Elliptic };
    Ok(blend::fillet_conic_edges(m, cylinder, &[round])?)
}
