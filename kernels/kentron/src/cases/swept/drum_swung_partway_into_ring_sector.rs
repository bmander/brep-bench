//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a drum swung partway into it: the solid cylinder of
//! radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, turned rigidly about the z axis
//! from −140° to +30°. Everything the drum passes through, cut.
//!
//! The drum's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let drum = Cylinder::new(1.0, 1.0).with_frame(frame(p(10.0, 0.0, -0.5), Direction::Z)).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, drum, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
