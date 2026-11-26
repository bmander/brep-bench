//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut round it by a ball of radius 1 whose centre follows the helix of radius 5 about the z axis
//! from (5, 0, −2), right-handed, rising 4 a turn, for six turns to (5, 0, 22): everything the
//! ball passes through, cut.
//!
//! The ball carried along the screw about the z axis keeps its centre on that helix; its sweep is
//! kentron's sweep of a solid (`sweep::solid`, one screw), cut from the rod.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::{Cylinder, Sphere};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let ball = Sphere::new(1.0).with_frame(frame(p(5.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 4.0, from: 0.0, to: 6.0 * TAU };
    let swept = sweep::solid(m, ball, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
