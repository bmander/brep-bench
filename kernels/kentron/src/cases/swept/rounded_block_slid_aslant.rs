//! The solid a rounded block fills as it slides, without turning, along (8, 5, 3): the block is
//! everything within 1 of the box from (1, 1, 1) to (9, 5, 3), the box from (0, 0, 0) to
//! (10, 6, 4) with every edge rounded to radius 1.
//!
//! The block is the box with all twelve edges filleted (`blend::fillet`); its sweep is kentron's
//! sweep of a solid (`sweep::solid`, one slide). kentron refuses it: following where the slide
//! grazes the block's rounds, a point does not settle on the curve.
use crate::bench::{edges, p, v};
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let box_ = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 6.0, 4.0)).build(m)?;
    let all = edges(m, box_);
    let block = blend::fillet(m, box_, &all, 1.0)?;
    sweep::solid(m, block, &RigidMotion::Slide { by: v(8.0, 5.0, 3.0) })
}
