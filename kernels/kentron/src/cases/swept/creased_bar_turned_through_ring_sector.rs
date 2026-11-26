//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, grooved by a bar drafted both ways from a parting plane, turned
//! past it: with u = (1, 0, 0), v = (0, cos 20°, −sin 20°) and n = (0, sin 20°, cos 20°), every
//! a u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2° and |b| ≤ 2 − |h| tan 2°,
//! turned rigidly about the z axis from −140° to +140°. Everything the bar passes through, cut.
//!
//! The bar is the parting rectangle extruded 4 each way along ±n with its sides drafted in 2°
//! (`sweep::extrude_drafted`), the two halves joined; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one turn), cut from the block.
use crate::bench::v;
use crate::shared::loft::flat;
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Point};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let (u, w, n) = (v(1.0, 0.0, 0.0), v(0.0, c20, -s20), v(0.0, s20, c20));
    let at = |a: f64, b: f64| -> Point { Point::ORIGIN + u * a + w * b };
    let parting = [at(10.0, -2.0), at(13.0, -2.0), at(13.0, 2.0), at(10.0, 2.0)];
    let draft = 2f64.to_radians();
    let mut halves = Vec::new();
    for side in [1.0, -1.0] {
        let face = flat(m, &parting)?;
        halves.push(sweep::extrude_drafted(m, face, n * (4.0 * side), draft)?);
    }
    let bar = kentron::boolean::union(m, halves[0], halves[1])?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, bar, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
