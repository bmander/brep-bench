//! The solid a thick disc fills as it is turned about an upright line through its rim: the solid
//! cylinder of radius 2 about the line through (2, 0, 0) along y, from y = 0 to 0.5, turned rigidly
//! about the z axis by 90°.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis
//! touches the coin's rim, and kentron's sweep refuses a turn about an axis meeting the tool.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let coin = Cylinder::new(2.0, 0.5).with_frame(frame(p(2.0, 0.0, 0.0), Direction::Y)).build(m)?;
    sweep::solid(m, coin, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: FRAC_PI_2 })
}
