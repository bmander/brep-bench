
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`) for kentron's sweep of a
//! solid (`sweep::solid`), which refuses it: the curves where the motion grazes the tool move over it, so its envelope is
//! no product of a fixed curve and the motion, and kentron's sweep builds only those.
use crate::bench::{frame, p, v};
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::Sphere;
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(1.0).with_frame(frame(p(5.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let place = |t: f64| Transform::translation(v(25.0 * t, 0.0, 0.0)) * Transform::rotation(Axis::Z, PI * t);
    sweep::solid(m, ball, &RigidMotion::Carried(Carriage(Arc::new(place))))
}
