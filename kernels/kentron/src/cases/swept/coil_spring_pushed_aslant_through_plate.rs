
//!
//! The spring is the ball carried along the screw about the z axis, its centre on the helix, and
//! the spring is pushed along the slide: each kentron's sweep of a solid (`sweep::solid`, a screw
//! and then a slide), cut from the plate.
use crate::bench::{frame, p, v};
use kentron::math::{Axis, Direction};
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 4.0)).build(m)?;
    let ball = Sphere::new(0.4).with_frame(frame(p(2.0, 0.0, -12.0), Direction::Z)).build(m)?;
    let spring = sweep::solid(m, ball, &RigidMotion::Screw { axis: Axis::Z, lead: 1.5, from: 0.0, to: 3.0 * TAU })?;
    let swept = sweep::solid(m, spring, &RigidMotion::Slide { by: v(3.0, 1.0, 18.0) })?;
    kentron::boolean::difference(m, plate, swept)
}
