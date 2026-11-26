//! A block, the box from (0, −5, 0) to (20, 5, 10), with a slot cut across its top by an end mill:
//! the solid cylinder of radius 2 about the vertical line through (−5, 0), from z = 7 up to 15,
//! slid without turning along the x axis by 30, from clear before the block to clear past it.
//! Everything the mill passes through, cut.
//!
//! The mill's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the block.
use crate::bench::{frame, p, v};
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 10.0)).build(m)?;
    let mill = Cylinder::new(2.0, 8.0).with_frame(frame(p(-5.0, 0.0, 7.0), Direction::Z)).build(m)?;
    let swept = sweep::solid(m, mill, &RigidMotion::Slide { by: v(30.0, 0.0, 0.0) })?;
    kentron::boolean::difference(m, block, swept)
}
