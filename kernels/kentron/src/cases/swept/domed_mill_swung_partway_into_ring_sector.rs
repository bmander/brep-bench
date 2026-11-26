//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a domed mill swung partway into it: about the
//! vertical line through (10, 0), a cylinder of radius 1 up to z = 3 ended below by the cap of the
//! sphere of radius 2 about (10, 0, 1.2), meeting it at z = 1.2 − √3; turned rigidly about the z
//! axis from −140° to +30°. Everything the mill passes through, cut.
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
    use Step::{Arc, Heading, Start, To};
    let block = ring_sector(m)?;
    // The mill: its half section, out from its axis (the cap's arc from its lowest point round
    // 30° to the rim), turned whole about it.
    let half = [Start(10.0, -0.8), Heading(0.0), Arc { radius: 2.0, angle: 30.0 }, To(11.0, 3.0), To(10.0, 3.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let mill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 30f64.to_radians() };
    let swept = sweep::solid(m, mill, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
