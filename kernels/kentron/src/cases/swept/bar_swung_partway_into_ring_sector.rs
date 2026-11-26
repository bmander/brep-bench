//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a bar swung partway into it: the box 10 ≤ x ≤ 11,
//! |y| ≤ 0.5, |z| ≤ 0.6, turned rigidly about the z axis from −140° to +30°. Everything the bar
//! passes through, cut.
//!
//! The bar's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::p;
use crate::shared::swept::ring_sector;
use kentron::math::Axis;
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let bar = Cuboid::from_corners(p(10.0, -0.5, -0.6), p(11.0, 0.5, 0.6)).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, bar, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
