//! A block, the box from (−8, −8, −8) to (8, 8, 0), with a hole bored by a ring pushed straight
//! down 8 into it at a lean: the torus centred at (0, 0, 4), its axis (sin 35°, 0, cos 35°), radii
//! 3 and 1.
//!
//! The ring's sweep is kentron's sweep of a solid (`sweep::solid`, one slide), cut from the block.
use crate::bench::{d, frame, p, v};
use kentron::primitives::{Cuboid, Torus};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::new(16.0, 16.0, 8.0).with_frame(frame(p(-8.0, -8.0, -8.0), d(0.0, 0.0, 1.0))).build(m)?;
    let lean = 35f64.to_radians();
    let ring = Torus::new(3.0, 1.0).with_frame(frame(p(0.0, 0.0, 4.0), d(lean.sin(), 0.0, lean.cos()))).build(m)?;
    let swept = sweep::solid(m, ring, &RigidMotion::Slide { by: v(0.0, 0.0, -8.0) })?;
    kentron::boolean::difference(m, block, swept)
}
