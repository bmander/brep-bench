//! A plate, the box from (0, −5, 0) to (20, 5, 4), pocketed from below by a drum pushed up aslant
//! into it: the solid cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14, slid
//! without turning by 8 along (0.3, 0, √0.91), stopping inside the plate. Everything the drum
//! passes through, cut.
//!
//! The drum's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the plate.
use crate::bench::{frame, p, v};
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    let drum = Cylinder::new(2.0, 8.0).with_frame(frame(p(6.0, 0.0, -5.0), Direction::X)).build(m)?;
    let swept = sweep::solid(m, drum, &RigidMotion::Slide { by: v(0.3, 0.0, 0.91f64.sqrt()) * 8.0 })?;
    kentron::boolean::difference(m, plate, swept)
}
