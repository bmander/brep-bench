//! The solid a ball fills as it is swung about an upright line through it: the ball of radius 2
//! about (1, 0, 0), turned rigidly about the z axis by 120°.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis
//! passes through the ball, and kentron's sweep refuses a turn about an axis meeting the tool
//! (a point on the axis stands still, and the curves where the motion grazes the tool meet there).
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Sphere;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(2.0).with_frame(frame(p(1.0, 0.0, 0.0), Direction::Z)).build(m)?;
    sweep::solid(m, ball, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: 120f64.to_radians() })
}
