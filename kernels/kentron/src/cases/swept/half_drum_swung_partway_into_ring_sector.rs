//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by half a drum swung partway into it: the half with
//! y ≤ 0 of the solid cylinder of radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, its
//! flat face the rectangle 9 ≤ x ≤ 11, y = 0, turned rigidly about the z axis from −140° to +30°.
//! Everything the half drum passes through, cut.
//!
//! The half drum's sweep is kentron's sweep of a solid (`sweep::solid`, one turn), cut from the
//! block.
use crate::bench::p;
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    // The half of the cylinder from polar angle 180° to 360° about its axis, the side facing −y.
    let axis = Frame::new(p(10.0, 0.0, -0.5), Direction::Z, Direction::NEG_X);
    let half = Cylinder::new(1.0, 1.0).with_angle(PI).with_frame(axis).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, half, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
