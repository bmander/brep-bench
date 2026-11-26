//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a square key, the box from
//! (−1.5, −0.8, −1.5) to (1.5, 0.8, 2.5) in a frame carried along the cubic Bézier curve through
//! control points (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0), its x axis the curve's
//! tangent and its z axis up.
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`) for kentron's sweep of a
//! solid (`sweep::solid`), which refuses it: the key's top and bottom faces are carried along
//! their own planes while the frame turns, beside faces it carries out, and their edges sweep
//! regions of those planes over one another, which kentron's sweep does not resolve.
use crate::bench::{d, frame, p};
use crate::shared::swept::bezier_frame;
use kentron::primitives::Cuboid;
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::new(42.0, 32.0, 8.0).with_frame(frame(p(-6.0, -6.0, -8.0), d(0.0, 0.0, 1.0))).build(m)?;
    let key = Cuboid::new(3.0, 1.6, 4.0).with_frame(frame(p(-1.5, -0.8, -1.5), d(0.0, 0.0, 1.0))).build(m)?;
    let swept = sweep::solid(m, key, &RigidMotion::Carried(Carriage(Arc::new(bezier_frame))))?;
    kentron::boolean::difference(m, block, swept)
}
