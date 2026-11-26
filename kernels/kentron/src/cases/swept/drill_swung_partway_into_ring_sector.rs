//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a drill swung partway into it: about the vertical
//! line through (10, 0), a cylinder of radius 1 up to z = 3 ended below by a cone of 118° whose
//! point is (10, 0, −0.8), turned rigidly about the z axis from −140° to +30°. Everything the
//! drill passes through, cut.
//!
//! The drill is its half section revolved about its axis; its sweep is kentron's sweep of a solid
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
    use Step::{Start, To};
    let block = ring_sector(m)?;
    // Where the cone meets the cylinder.
    let rim = -0.8 + 1.0 / 59f64.to_radians().tan();
    let half = [Start(10.0, -0.8), To(11.0, rim), To(11.0, 3.0), To(10.0, 3.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let drill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, drill, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
