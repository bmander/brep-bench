//! The solid a square bar fills as it is carried round on a planet: the prism from z = 0 to z = 2
//! on the square of side 2 centred at (5, 0), its sides along x and y, its centre carried round
//! the circle of radius 5 about the z axis to (5 cos t, 5 sin t) while it turns about its centre
//! by −2t, for t from 0° to 360°.
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`): the bar's turn about its
//! own centre by −3t, then the carrier's turn by t about the z axis. Its sweep is kentron's sweep
//! of a solid (`sweep::solid`), which refuses it: as the bar turns against the carrier, the lines
//! where the motion grazes its sides move over them, so its envelope is no product of a fixed
//! curve and the motion, and kentron's sweep builds only those.
use crate::bench::p;
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let bar = Cuboid::from_corners(p(4.0, -1.0, 0.0), p(6.0, 1.0, 2.0)).build(m)?;
    let place = |s: f64| {
        let t = s * TAU;
        Transform::rotation(Axis::Z, t) * Transform::rotation(Axis::new(p(5.0, 0.0, 0.0), Direction::Z), -3.0 * t)
    };
    sweep::solid(m, bar, &RigidMotion::Carried(Carriage(Arc::new(place))))
}
