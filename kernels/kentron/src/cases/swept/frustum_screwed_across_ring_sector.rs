//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, with a helical groove milled through it by a tapered cutter:
//! the frustum about the vertical line through (10, 0), of radius 0.4 at z = −0.6 widening to 1 at
//! z = 1.4, carried along the screw about the z axis, right-handed, rising 2 a turn, from a turn
//! of −140° to +140°. Everything the cutter passes through, cut.
//!
//! The cutter's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the block.
use crate::bench::{d, frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cone;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let cutter = Cone::new(0.4, 1.0, 2.0).with_frame(frame(p(10.0, 0.0, -0.6), d(0.0, 0.0, 1.0))).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), lead: 2.0, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, cutter, &screw)?;
    kentron::boolean::difference(m, block, swept)
}
