//! A block, the box from (−3, −3, −3) to (3, 3, 3), with a sealed void inside it: everything the
//! cube from (−1, −1, −1) to (1, 1, 1) passes through as it turns a whole turn about its own long
//! diagonal, cut.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis runs
//! through the cube, and kentron's sweep refuses a turn about an axis meeting the tool, as it
//! refuses a whole turn (its ends would be one).
use crate::bench::{d, p};
use kentron::math::Axis;
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-3.0, -3.0, -3.0), p(3.0, 3.0, 3.0)).build(m)?;
    let cube = Cuboid::from_corners(p(-1.0, -1.0, -1.0), p(1.0, 1.0, 1.0)).build(m)?;
    let diagonal = Axis::new(p(0.0, 0.0, 0.0), d(1.0, 1.0, 1.0));
    let void = sweep::solid(m, cube, &RigidMotion::Turn { axis: diagonal, from: 0.0, to: TAU })?;
    kentron::boolean::difference(m, block, void)
}
