//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar whose
//! section is a spline, carried along a screw: the closed uniform periodic cubic B-spline in the
//! plane y = 0 through the control points (x, z) (3.5, 0), (4.5, −1.5), (8, −1.5), (12, −1),
//! (12, 1), (8, 1.5), (4.5, 1.5), carried from y = −8 to y = 8, screwed about the z axis,
//! right-handed, rising 6 a turn, from a turn of −90° to +1350°. Everything the bar passes
//! through, cut.
//!
//! The bar is the spline's face extruded; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::math::{Axis, Direction, Vector};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let xz = [(3.5, 0.0), (4.5, -1.5), (8.0, -1.5), (12.0, -1.0), (12.0, 1.0), (8.0, 1.5), (4.5, 1.5)];
    let poles = xz.iter().map(|&(x, z)| p(x, -8.0, z)).collect();
    let knots = (0..=xz.len()).map(|k| k as f64).collect();
    let section = BSplineCurve::try_new_periodic(poles, knots, vec![1; xz.len() + 1], 3)?;
    let edge = make::edge_from_curve(m, Curve::BSpline(section))?;
    let wire = make::wire(m, &[edge])?;
    let face = make::planar_face(m, wire)?;
    let bar = sweep::extrude(m, face, Vector::new(0.0, 16.0, 0.0))?.shape;
    let screw = RigidMotion::Screw { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), lead: 6.0, from: -FRAC_PI_2, to: 1350f64.to_radians() };
    let swept = sweep::solid(m, bar, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
