//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, cut by a leaning bar turned past it: the box from its inner
//! side, the plane through (10, 0, 0) square to u = (cos 20°, 0, −sin 20°), to 3 beyond it along
//! u, within 2 of the plane y = 0 and within 3 along its length ℓ = (sin 20°, 0, cos 20°) of
//! (10, 0, 0), turned rigidly about the z axis from −140° to +140°. Everything the bar passes
//! through, cut.
//!
//! The bar is a box in the frame of u, y and ℓ; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one turn), cut from the block.
use crate::bench::{d, p, v};
use crate::shared::swept::ring_sector;
use kentron::math::{Axis, Frame};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = ring_sector(m)?;
    let (sin, cos) = 20f64.to_radians().sin_cos();
    // x along u, y along y, z along u × y = ℓ; from the corner 3 back along ℓ and 2 along −y.
    let corner = p(10.0, 0.0, 0.0) - v(sin, 0.0, cos) * 3.0 - v(0.0, 2.0, 0.0);
    let bar = Cuboid::new(3.0, 4.0, 6.0).with_frame(Frame::new(corner, d(sin, 0.0, cos), d(cos, 0.0, -sin))).build(m)?;
    let turn = RigidMotion::Turn { axis: Axis::Z, from: (-140f64).to_radians(), to: 140f64.to_radians() };
    let swept = sweep::solid(m, bar, &turn)?;
    kentron::boolean::difference(m, block, swept)
}
