//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre follows the cubic Bézier curve in the top face with control
//! points (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0): everything the ball passes through, cut.
//!
//! The ball is carried in the frame riding the curve (`shared::swept::bezier_frame`), its centre
//! the curve's point (how the ball turns about its centre changes nothing it fills); the motion is
//! given by its places (`sweep::RigidMotion::Carried`) for kentron's sweep of a solid
//! (`sweep::solid`), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::bezier_frame;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Sphere};
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-6.0, -6.0, -8.0), p(36.0, 26.0, 0.0)).build(m)?;
    let ball = Sphere::new(3.0).with_frame(frame(p(0.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let swept = sweep::solid(m, ball, &RigidMotion::Carried(Carriage(Arc::new(bezier_frame))))?;
    kentron::boolean::difference(m, block, swept)
}
