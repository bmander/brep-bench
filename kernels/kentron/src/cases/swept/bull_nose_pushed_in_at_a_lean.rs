//! A block, the box from (−8, −8, −8) to (8, 8, 0), bored into from its top by a leaning bull-nose
//! mill pushed straight down: about the axis through c = (0, 0, 4) along a = (sin 30°, 0, cos 30°),
//! from c to c + 3a, a cylinder of radius 2 whose end corner is rounded to 0.5, slid without
//! turning down by 8. Everything the mill passes through, cut.
//!
//! The mill is its meridian revolved, leaned and placed; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one slide), cut from the block.
use crate::bench::{d, frame, p, v};
use crate::shared::swept::bull_nose;
use kentron::math::{Axis, Transform, Vector};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::new(16.0, 16.0, 8.0).with_frame(frame(p(-8.0, -8.0, -8.0), d(0.0, 0.0, 1.0))).build(m)?;
    let place = Transform::translation(Vector::new(0.0, 0.0, 4.0)) * Transform::rotation(Axis::Y, 30f64.to_radians());
    let mill = bull_nose(m, &place)?;
    let swept = sweep::solid(m, mill, &RigidMotion::Slide { by: v(0.0, 0.0, -8.0) })?;
    kentron::boolean::difference(m, block, swept)
}
