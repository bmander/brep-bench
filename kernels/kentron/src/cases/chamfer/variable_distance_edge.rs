//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet chamfered with a distance on the face x = 10 growing linearly along it,
//! d₁ = 1 + z/5, and a constant 2 on the face y = 10: a hyperbolic paraboloid.
use crate::bench::{edge_through, face_through, p};
use crate::Refusal;
use kentron::blend::{self, EvolvingChamfer};
use kentron::geometry::ParametricCurve;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, cube, p(10.0, 10.0, 5.0))?;
    let side = face_through(m, cube, p(10.0, 5.0, 5.0))?;
    // The law runs from the edge's start to its end by length, which here is by height: each end
    // gets the distance at its height.
    let (curve, first, _) = m.edge_curve(edge).ok_or("the edge has no curve")?;
    let growing = if curve.point(first).z < 5.0 { vec![(0.0, 1.0), (1.0, 3.0)] } else { vec![(0.0, 3.0), (1.0, 1.0)] };
    let chamfer = EvolvingChamfer { edge, face: Some(side), first: growing, second: vec![(0.0, 2.0)] };
    Ok(blend::chamfer_evolving(m, cube, &[chamfer])?)
}
