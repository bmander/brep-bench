//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar
//! carried along a fine screw: the box 4.5 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 0.45, screwed about the z axis,
//! right-handed, rising 1 a turn, from a turn of −180° to +7380°. Everything the bar passes
//! through, cut.
//!
//! The bar's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::math::Axis;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let bar = Cuboid::from_corners(p(4.5, -8.0, -0.45), p(12.0, 8.0, 0.45)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 1.0, from: (-180f64).to_radians(), to: 7380f64.to_radians() };
    let swept = sweep::solid(m, bar, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
