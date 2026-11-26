//! The solid a ring fills as it turns a whole turn about the z axis: the torus whose axis is the
//! vertical line through (8, 0), the circle through its tube's centres of radius 3, its tube of
//! radius 1, centred on the plane z = 0.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: kentron's sweep
//! refuses a turn of a whole turn or more, whose ends would be one.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Torus;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ring = Torus::new(3.0, 1.0).with_frame(frame(p(8.0, 0.0, 0.0), Direction::Z)).build(m)?;
    sweep::solid(m, ring, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: TAU })
}
