//! The solid a rod fills as it is spun partway about an upright line through it: the solid
//! cylinder of radius 0.5 about the x axis from x = −3 to x = 5, turned rigidly about the z axis
//! by 120°.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis passes
//! through the rod, and kentron's sweep refuses a turn about an axis meeting the tool.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(0.5, 8.0).with_frame(frame(p(-3.0, 0.0, 0.0), Direction::X)).build(m)?;
    sweep::solid(m, rod, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: 120f64.to_radians() })
}
