//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by a bar, the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, carried along a screw about
//! the z axis rising 6 a turn, from a turn of −120° (wholly below the rod) to 600°, where it stops
//! inside the rod. Everything the bar passes through, cut.
//!
//! The bar's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the rod.
use crate::bench::p;
use kentron::math::Axis;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let bar = Cuboid::from_corners(p(4.0, -8.0, -1.0), p(12.0, 8.0, 1.0)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 6.0, from: (-120f64).to_radians(), to: 600f64.to_radians() };
    let swept = sweep::solid(m, bar, &screw)?;
    kentron::boolean::difference(m, rod, swept)
}
