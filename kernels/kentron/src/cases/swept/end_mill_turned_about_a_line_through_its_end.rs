//! A block, the box from (−10, −10, −5) to (10, 10, 0), pocketed by an end mill turned about a
//! line through its own flat end: the solid cylinder of radius 3 about the vertical line through
//! (1.5, 0), from z = −2 to 4, turned rigidly about the z axis by 120°. Everything the mill passes
//! through, cut.
//!
//! kentron's sweep of a solid (`sweep::solid`, one turn), which refuses it: the turn's axis passes
//! through the mill, and kentron's sweep refuses a turn about an axis meeting the tool.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, -5.0), p(10.0, 10.0, 0.0)).build(m)?;
    let mill = Cylinder::new(3.0, 6.0).with_frame(frame(p(1.5, 0.0, -2.0), Direction::Z)).build(m)?;
    let swept = sweep::solid(m, mill, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: 120f64.to_radians() })?;
    kentron::boolean::difference(m, block, swept)
}
