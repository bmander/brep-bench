//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a ring lying flat swung partway into it: the torus
//! about the vertical line through (10, 0), radii 1 and 0.4, centred on the plane z = 0, turned
//! rigidly about the z axis from −140° to +30°. Everything the ring passes through, cut.
//!
//! The ring's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::primitives::Torus;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let ring = Torus::new(1.0, 0.4).with_frame(frame(p(10.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, ring, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
