//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a sealed helical
//! tunnel inside it: everything the box 2.5 ≤ x ≤ 3.5, |y| ≤ 0.5, 2 ≤ z ≤ 3 passes through as it is
//! carried along the screw about the z axis, right-handed, rising 4 a turn, from a turn of 0° to
//! 720°, cut.
//!
//! The box's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::math::Axis;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let block = Cuboid::from_corners(p(2.5, -0.5, 2.0), p(3.5, 0.5, 3.0)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 4.0, from: 0.0, to: 2.0 * TAU };
    let swept = sweep::solid(m, block, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
