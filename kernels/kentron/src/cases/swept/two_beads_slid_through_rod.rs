//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two beads slid past it: tori on one axis parallel to the y axis, the first centred at
//! (11.5, 0, −9) with radii 6 and 2, the second at (11.5, 1.5, −9) with radii 6 and 1.5, their
//! union slid 40 along the z axis from below the rod to above it. Everything the wheel passes
//! through, cut.
//!
//! The wheel is the beads' union; its sweep is kentron's sweep of a solid (`sweep::solid`, one
//! slide), cut from the rod.
use crate::bench::{d, frame, p, v};
use kentron::primitives::{Cylinder, Torus};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let first = Torus::new(6.0, 2.0).with_frame(frame(p(11.5, 0.0, -9.0), d(0.0, 1.0, 0.0))).build(m)?;
    let second = Torus::new(6.0, 1.5).with_frame(frame(p(11.5, 1.5, -9.0), d(0.0, 1.0, 0.0))).build(m)?;
    let wheel = kentron::boolean::union(m, first, second)?;
    let swept = sweep::solid(m, wheel, &RigidMotion::Slide { by: v(0.0, 0.0, 40.0) })?;
    kentron::boolean::difference(m, rod, swept)
}
