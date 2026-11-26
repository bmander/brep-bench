//! A block, the box from (0, −10, −10) to (30, 10, 0), engraved by a V bit that stops inside it at
//! both ends: the solid cone about the vertical line through (5, 0), its point at (5, 0, −2),
//! opening upward at 90° to a flat top of radius 5 at z = 3, slid without turning along the x axis
//! by 15. Everything the bit passes through, cut.
//!
//! The bit's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the block.
use crate::bench::{frame, p, v};
use kentron::math::Direction;
use kentron::primitives::{Cone, Cuboid};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -10.0, -10.0), p(30.0, 10.0, 0.0)).build(m)?;
    let bit = Cone::new(0.0, 5.0, 5.0).with_frame(frame(p(5.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let swept = sweep::solid(m, bit, &RigidMotion::Slide { by: v(15.0, 0.0, 0.0) })?;
    kentron::boolean::difference(m, block, swept)
}
