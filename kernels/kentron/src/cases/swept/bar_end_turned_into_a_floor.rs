//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, its floor cut by the flat end of a leaning bar turned past it:
//! in the plane through (10, 0, 0) spanned by u = (cos 20°, 0, −sin 20°) and v = (0, 1, 0), the
//! 3 × 4 rectangle 0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2 of points (a, b), carried 3
//! along n = (sin 20°, 0, cos 20°), turned rigidly about the z axis from −140° to +140°.
//! Everything the bar passes through, cut.
//!
//! The bar is a box in the frame of the rectangle's sides and n; its sweep is kentron's sweep of a
//! solid (`sweep::solid`, one turn), cut from the block.
use crate::bench::{d, v};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Frame, Point, Vector};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let (s5, c5) = 5f64.to_radians().sin_cos();
    let (u, w, n) = (v(c20, 0.0, -s20), v(0.0, 1.0, 0.0), v(s20, 0.0, c20));
    // The rectangle's sides' directions, 5° round from u and v (n × e1 = e2); its corner where
    // a cos 5° + b sin 5° is 0 and b cos 5° − a sin 5° is −2.
    let (e1, e2) = (u * c5 + w * s5, w * c5 - u * s5);
    let corner = Point::new(10.0, 0.0, 0.0) - e2 * 2.0;
    let unit = |a: Vector| d(a.x, a.y, a.z);
    let bar = Cuboid::new(3.0, 4.0, 3.0).with_frame(Frame::new(corner, unit(n), unit(e1))).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, bar, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
