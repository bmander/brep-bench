//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, with a helical groove milled through it by a drum leaning to
//! the screw: the solid cylinder of radius 0.8 and length 3 whose axis runs along
//! (0, sin 25°, cos 25°) through its middle (10, 0, 0), carried along the screw about the z axis,
//! right-handed, rising 1 a turn, from a turn of −140° to +140°. Everything the drum passes
//! through, cut.
//!
//! The drum's sweep is kentron's sweep of a solid (`sweep::solid`, one screw), cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction, Vector};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let lean = 25f64.to_radians();
    let axis = Vector::new(0.0, lean.sin(), lean.cos());
    let drum = Cylinder::new(0.8, 3.0).with_frame(frame(p(10.0, 0.0, 0.0) - axis * 1.5, Direction::from_vector(axis))).build(m)?;
    let screw = RigidMotion::Screw { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), lead: 1.0, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, drum, &screw)?;
    kentron::boolean::difference(m, block, swept)
}
