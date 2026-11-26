
//!
//! The ball carried along the screw about the z axis keeps its centre on the helix; its sweep is
//! kentron's sweep of a solid (`sweep::solid`, one screw), which refuses it: the helix turns on a
//! radius of about 1.65, under the ball's 2, so the sweep folds over itself.
use crate::bench::{frame, p};
use kentron::math::{Axis, Direction};
use kentron::primitives::Sphere;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(2.0).with_frame(frame(p(1.5, 0.0, 0.0), Direction::Z)).build(m)?;
    sweep::solid(m, ball, &RigidMotion::Screw { axis: Axis::Z, lead: 3.0, from: 0.0, to: 2.0 * TAU })
}
