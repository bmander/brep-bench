//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched by a tilted box pushed
//! aslant wholly through it: the 4 × 3 × 2 box (−2, −1, −8) + a e₁ + b e₂ + c e₃, |a| ≤ 2, |b| ≤
//! 1.5, |c| ≤ 1, its edges along e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3 and e₃ = (1, 2, −2)/3, slid
//! without turning along (4, 2, 18), from wholly below the plate to wholly above it. Everything the
//! box passes through, cut.
//!
//! The box's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the plate.
use crate::bench::{d, p, v};
use kentron::math::Frame;
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 4.0)).build(m)?;
    // The box's frame: x along e₁, y along e₂ and z along e₁ × e₂ = −e₃, at its lowest corner.
    let (e1, e2, e3) = (v(2.0, 1.0, 2.0) / 3.0, v(-2.0, 2.0, 1.0) / 3.0, v(1.0, 2.0, -2.0) / 3.0);
    let corner = p(-2.0, -1.0, -8.0) - e1 * 2.0 - e2 * 1.5 + e3;
    let tilted = Cuboid::new(4.0, 3.0, 2.0).with_frame(Frame::new(corner, d(-e3.x, -e3.y, -e3.z), d(e1.x, e1.y, e1.z))).build(m)?;
    let swept = sweep::solid(m, tilted, &RigidMotion::Slide { by: v(4.0, 2.0, 18.0) })?;
    kentron::boolean::difference(m, plate, swept)
}
