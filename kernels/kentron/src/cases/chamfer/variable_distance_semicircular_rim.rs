//! The half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 with x ≥ 0, the half
//! circle where its curved side meets its top chamfered by the same distance on both faces,
//! growing linearly with the angle round the axis, d = 3/2 + φ/π: a ruled surface between a
//! spiral of Archimedes on the top and a helix on the side.
use crate::bench::{d, edge_through, p};
use crate::Refusal;
use kentron::blend::{self, EvolvingChamfer};
use kentron::geometry::ParametricCurve;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // The cylinder primitive turned through half a turn from −y, counter-clockwise through +x.
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, d(0.0, -1.0, 0.0));
    let half = Cylinder::new(5.0, 6.0).with_frame(frame).with_angle(PI).build(m)?;
    let rim = edge_through(m, half, p(5.0, 0.0, 6.0))?;
    // The law runs from the edge's start to its end by length, which on a circle is by angle:
    // each end gets the distance at its angle.
    let (curve, first, _) = m.edge_curve(rim).ok_or("the rim has no curve")?;
    let law = if curve.point(first).y < 0.0 { vec![(0.0, 1.0), (1.0, 2.0)] } else { vec![(0.0, 2.0), (1.0, 1.0)] };
    let chamfer = EvolvingChamfer { edge: rim, face: None, first: law.clone(), second: law };
    Ok(blend::chamfer_evolving(m, half, &[chamfer])?)
}
