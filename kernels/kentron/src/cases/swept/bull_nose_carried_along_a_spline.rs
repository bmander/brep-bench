//! A block, the box from (−6, −6, −8) to (36, 26, 0), grooved by a leaning bull-nose mill carried
//! along a spline: about the axis through c = (0, 0, −1) along (sin 30°, 0, cos 30°), from c up 3,
//! a cylinder of radius 2 whose end corner is rounded to 0.5, in a frame carried along the cubic
//! Bézier curve through (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0), its x axis the
//! curve's tangent and its z axis up.
//!
//! The motion is given by its places (`sweep::RigidMotion::Carried`) for kentron's sweep of a
//! solid (`sweep::solid`), which refuses it: as the frame turns, the curves where the mill's
//! round and side touch what it sweeps move over them, so the groove's walls are no product of a
//! fixed curve and the motion, and kentron's sweep builds only those.
use crate::bench::{d, frame, p};
use crate::shared::swept::{bezier_frame, bull_nose};
use kentron::math::{Axis, Transform, Vector};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::sync::Arc;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::new(42.0, 32.0, 8.0).with_frame(frame(p(-6.0, -6.0, -8.0), d(0.0, 0.0, 1.0))).build(m)?;
    let place = Transform::translation(Vector::new(0.0, 0.0, -1.0)) * Transform::rotation(Axis::Y, 30f64.to_radians());
    let mill = bull_nose(m, &place)?;
    let swept = sweep::solid(m, mill, &RigidMotion::Carried(Carriage(Arc::new(bezier_frame))))?;
    kentron::boolean::difference(m, block, swept)
}
