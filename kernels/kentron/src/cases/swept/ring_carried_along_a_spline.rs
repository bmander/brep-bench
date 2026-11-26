
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`, the frame riding the curve,
//! `shared::swept::bezier_frame`) for kentron's sweep of a solid (`sweep::solid`), cut from the
//! block. kentron's sweep refuses it: the curves where the motion grazes the ring move over it, so
//! its envelope is no product of a fixed curve and the motion.
use crate::bench::{frame, p};
use crate::shared::swept::bezier_frame;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Torus};
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-6.0, -6.0, -8.0), p(36.0, 26.0, 0.0)).build(m)?;
    let ring = Torus::new(1.5, 0.5).with_frame(frame(p(0.0, 0.0, 0.5), Direction::Y)).build(m)?;
    let swept = sweep::solid(m, ring, &RigidMotion::Carried(Carriage(Arc::new(bezier_frame))))?;
    kentron::boolean::difference(m, block, swept)
}
