//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel slid
//! past it: the torus with its axis parallel to the y axis, radii 6 and 2, slid without turning
//! from centre (11.5, 0, −9), wholly below the rod, to (11.5, 0, 29), wholly above it. Everything
//! the wheel passes through, cut.
//!
//! The wheel's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the rod.
use crate::bench::{frame, p, v};
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Torus};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let wheel = Torus::new(6.0, 2.0).with_frame(frame(p(11.5, 0.0, -9.0), Direction::Y)).build(m)?;
    let swept = sweep::solid(m, wheel, &RigidMotion::Slide { by: v(0.0, 0.0, 38.0) })?;
    kentron::boolean::difference(m, rod, swept)
}
