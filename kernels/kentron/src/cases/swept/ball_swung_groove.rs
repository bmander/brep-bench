//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre swings a quarter turn about the z axis, right-handed, from
//! (12, 0, 0) to (0, 12, 0), staying in the top face: everything the ball passes through, cut.
//!
//! The ball's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-6.0, -6.0, -8.0), p(18.0, 18.0, 0.0)).build(m)?;
    let ball = Sphere::new(3.0).with_frame(frame(p(12.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let swept = sweep::solid(m, ball, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: FRAC_PI_2 })?;
    kentron::boolean::difference(m, block, swept)
}
