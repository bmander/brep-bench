//! The solid a ring fills as it slides across, square to its own axis, farther than the width of
//! its hole: the torus about the z axis centred on the origin, radii 6 and 2, slid without turning
//! along the x axis by 15.
//!
//! kentron's sweep of a solid (`sweep::solid`, one slide).
use crate::bench::v;
use kentron::primitives::Torus;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ring = Torus::new(6.0, 2.0).build(m)?;
    sweep::solid(m, ring, &RigidMotion::Slide { by: v(15.0, 0.0, 0.0) })
}
