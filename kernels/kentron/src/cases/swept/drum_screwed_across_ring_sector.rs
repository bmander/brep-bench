//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, with a helical groove milled through it by a drum: the solid
//! cylinder of radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, carried along the
//! screw about the z axis, right-handed, rising 0.5 a turn, from a turn of −140° to +140°.
//! Everything the drum passes through, cut.
//!
//! The drum's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let drum = Cylinder::new(1.0, 1.0).with_frame(frame(p(10.0, 0.0, -0.5), Direction::Z)).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::Z, lead: 0.5, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, drum, &screw)?;
    kentron::boolean::difference(m, block, swept)
}
