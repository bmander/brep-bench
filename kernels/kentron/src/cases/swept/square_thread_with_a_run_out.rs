//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by a bar, the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, carried along a screw about
//! the z axis rising 6 a turn, from a turn of −90° (wholly below the rod) to 900°, where it points
//! along −x, its tip at (−4, 0, 15), and then slid along −x by 3, out of its groove. Everything the
//! bar passes through, cut.
//!
//! The bar's two moves are each kentron's sweep of a solid (`sweep::solid`): the screw, and the
//! slide from where the screw leaves it. Both are cut from the rod.
use crate::bench::{p, v};
use kentron::math::Axis;
use kentron::modify::{self, TransformOptions};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let bar = Cuboid::from_corners(p(4.0, -8.0, -1.0), p(12.0, 8.0, 1.0)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 6.0, from: (-90f64).to_radians(), to: 900f64.to_radians() };
    let screwed = sweep::solid(m, bar, &screw)?;
    let stopped = modify::transform(m, bar, &screw.at(1.0), TransformOptions::default())?;
    let run_out = sweep::solid(m, stopped, &RigidMotion::Slide { by: v(-3.0, 0.0, 0.0) })?;
    let rod = kentron::boolean::difference(m, rod, screwed)?;
    kentron::boolean::difference(m, rod, run_out)
}
