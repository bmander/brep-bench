//! The solid the box from (0, 0, 0) to (10, 6, 4) fills as it slides, without turning, along
//! (8, 5, 3) to the box from (8, 5, 3) to (18, 11, 7).
//!
//! kentron's sweep of a solid (`sweep::solid`, one slide).
use crate::bench::{p, v};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 6.0, 4.0)).build(m)?;
    sweep::solid(m, block, &RigidMotion::Slide { by: v(8.0, 5.0, 3.0) })
}
