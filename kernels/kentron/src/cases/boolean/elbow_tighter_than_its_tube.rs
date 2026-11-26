//! A pipe elbow's outside, its bend tighter than the pipe is thick: the disk of radius 20 about
//! (0, 0, 18) in the plane x = 0 turned a quarter turn about the y axis (+z toward +x), its centre
//! running round the circle of radius 18, so that it reaches 2 past the axis; joined to a straight
//! leg of radius 20 and length 40 at each end, along y = 0, z = 18 from x = −40 to 0 and along
//! x = 18, y = 0 from z = −40 to 0.
use crate::bench::{frame, p};
use crate::shared::geometry::flat;
use kentron::boolean;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rim = make::edge_from_curve(m, Curve::Circle(Circle::new(frame(p(0.0, 0.0, 18.0), Direction::X), 20.0)))?;
    let disk = flat(m, &[rim])?;
    let turn = sweep::revolve(m, disk, Axis::Y, FRAC_PI_2)?.shape;
    let first = Cylinder::new(20.0, 40.0).with_frame(frame(p(-40.0, 0.0, 18.0), Direction::X)).build(m)?;
    let second = Cylinder::new(20.0, 40.0).with_frame(frame(p(18.0, 0.0, -40.0), Direction::Z)).build(m)?;
    let elbow = boolean::union(m, turn, first)?;
    boolean::union(m, elbow, second)
}
