//! A plate, the box from (0, −5, 0) to (20, 5, 4), pocketed from below by a bar pushed up aslant
//! into it: the prism 6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1 (a square stood on a corner), slid without
//! turning by 8 along (0.3, 0, √0.91), stopping inside the plate. Everything the bar passes
//! through, cut.
//!
//! The bar's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the plate.
use crate::bench::{d, p, v};
use kentron::math::{Direction, Frame};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    // The bar: a √2 square along x from its lowest edge (y, z) = (0, −7), its sides along
    // (0, 1, 1) and (0, −1, 1).
    let bar = Cuboid::new(2f64.sqrt(), 2f64.sqrt(), 8.0).with_frame(Frame::new(p(6.0, 0.0, -7.0), Direction::X, d(0.0, 1.0, 1.0))).build(m)?;
    let swept = sweep::solid(m, bar, &RigidMotion::Slide { by: v(0.3, 0.0, 0.91f64.sqrt()) * 8.0 })?;
    kentron::boolean::difference(m, plate, swept)
}
