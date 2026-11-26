//! A thick disc, the cylinder of radius 100 about the z axis from z = −20 to 20, counterbored
//! (the cylinder of radius 60 from z = 0 up past the top cut away), with a countersunk hole
//! drilled from the counterbore's floor 0.1 inside its wall, so that it reaches under the wall:
//! the hole, the cylinder of radius 3.5 about the upright through (59.9, 0) from z = −15 up to the
//! floor, and the countersink, the cone of 90° about the same axis from its point at z = −4 up to
//! its circle of radius 4 on the floor, both flush with the floor and cut away.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let disc = Cylinder::new(100.0, 40.0).with_frame(frame(p(0.0, 0.0, -20.0), Direction::Z)).build(m)?;
    let counterbore = Cylinder::new(60.0, 30.0).build(m)?;
    let disc = boolean::difference(m, disc, counterbore)?;
    let hole = Cylinder::new(3.5, 15.0).with_frame(frame(p(59.9, 0.0, -15.0), Direction::Z)).build(m)?;
    let sink = Cone::new(0.0, 4.0, 4.0).with_frame(frame(p(59.9, 0.0, -4.0), Direction::Z)).build(m)?;
    let disc = boolean::difference(m, disc, hole)?;
    boolean::difference(m, disc, sink)
}
