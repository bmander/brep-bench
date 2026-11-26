//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, slotted by a pin swung through it: the solid cylinder of radius
//! 0.5 about the x axis from x = 6 to x = 11, turned rigidly about the z axis from −140° to +140°.
//! Everything the pin passes through, cut.
//!
//! The pin's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let pin = Cylinder::new(0.5, 5.0).with_frame(frame(p(6.0, 0.0, 0.0), Direction::X)).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, pin, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
