//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a bull-nose mill swung partway into it: about the
//! vertical line through (10, 0), from z = −0.5 to 0.5, a cylinder of radius 1 whose bottom corner
//! is rounded to radius 0.3, turned rigidly about the z axis from −140° to +30°. Everything the
//! mill passes through, cut.
//!
//! The mill is its half section revolved about its axis; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one turn), cut from the block.
use crate::bench::p;
use crate::shared::parts::xz_plane;
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Direction};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let block = ring_sector(m)?;
    // The mill: its half section, out from its axis (the flat bottom to 0.7, the round's quarter
    // circle about (10.7, −0.2), the side up to the flat top), turned whole about it.
    let half = [Start(10.0, -0.5), To(10.7, -0.5), Arc { radius: 0.3, angle: 90.0 }, To(11.0, 0.5), To(10.0, 0.5)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let mill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, mill, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
