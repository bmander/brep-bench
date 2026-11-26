//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, threaded by the bar
//! 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 turned by θ about the z axis while rising 6s + 0.3s²,
//! s = θ/360°, for θ from −90° to 1260°.
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`); the bar's sweep is
//! kentron's sweep of a solid (`sweep::solid`), cut from the rod.
use crate::bench::{d, frame, p, v};
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let bar = Cuboid::new(8.0, 16.0, 2.0).with_frame(frame(p(4.0, -8.0, -1.0), d(0.0, 0.0, 1.0))).build(m)?;
    let (from, to) = ((-90f64).to_radians(), 1260f64.to_radians());
    let place = move |t: f64| {
        let theta = from + (to - from) * t;
        let s = theta / TAU;
        Transform::translation(v(0.0, 0.0, 6.0 * s + 0.3 * s * s)) * Transform::rotation(Axis::new(p(0.0, 0.0, 0.0), Direction::Z), theta)
    };
    let swept = sweep::solid(m, bar, &RigidMotion::Carried(Carriage(Arc::new(place))))?;
    kentron::boolean::difference(m, rod, swept)
}
