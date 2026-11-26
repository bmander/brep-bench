//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a ball on a leaning stem swung partway into it: the
//! ball of radius 0.8 about (10, 0, 0) and the solid cylinder of radius 0.4 from its centre 3
//! along (0, sin 20°, cos 20°), united, turned right-handed about the z axis from −140° to +30°.
//! Everything the tool passes through, cut.
//!
//! The tool passes through a point when the ball or the stem does, so its sweep is the union of
//! theirs, each kentron's sweep of a solid (`sweep::solid`, one turn); cut from the block.
use crate::bench::{frame, p};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction, Vector};
use kentron::primitives::{Cylinder, Sphere};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let lean = 20f64.to_radians();
    let axis = Direction::from_vector(Vector::new(0.0, lean.sin(), lean.cos()));
    let ball = Sphere::new(0.8).with_frame(frame(p(10.0, 0.0, 0.0), axis)).build(m)?;
    let stem = Cylinder::new(0.4, 3.0).with_frame(frame(p(10.0, 0.0, 0.0), axis)).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let ball_swept = sweep::solid(m, ball, &turn)?;
    let stem_swept = sweep::solid(m, stem, &turn)?;
    let swept = kentron::boolean::union(m, ball_swept, stem_swept)?;
    kentron::boolean::difference(m, block, swept)
}
