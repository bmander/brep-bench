
//!
//! The mill is its half section revolved about its axis. It is carried in the frame riding the
//! curve (`shared::swept::level_bezier_frame`), its axis on the curve (how it turns about its own
//! axis changes nothing it fills); the motion is given by its places
//! (`sweep::RigidMotion::Carried`) for kentron's sweep of a solid (`sweep::solid`), cut from the
//! block. kentron's sweep refuses it: the mill's flat end turns in its own plane beside faces the
//! motion carries out, and that end's edges sweep regions of its plane over one another.
use crate::bench::p;
use crate::shared::parts::xz_plane;
use crate::shared::swept::level_bezier_frame;
use kentron::math::Axis;
use kentron::primitives::Cuboid;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, Carriage, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;
use std::sync::Arc;

const CURVE: [(f64, f64); 4] = [(0.0, 0.0), (30.0, 20.0), (-10.0, 20.0), (20.0, 0.0)];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc as Round, Start, To};
    let block = Cuboid::from_corners(p(-10.0, -10.0, -8.0), p(30.0, 25.0, 0.0)).build(m)?;
    // The mill: its half section, out from its axis (the flat bottom to 1.5, the round's quarter
    // circle about (1.5, −1), the side up to the top), turned whole about it.
    let half = [Start(0.0, -1.5), To(1.5, -1.5), Round { radius: 0.5, angle: 90.0 }, To(2.0, 5.0), To(0.0, 5.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let mill = sweep::revolve(m, half, Axis::Z, TAU)?.shape;
    let along = RigidMotion::Carried(Carriage(Arc::new(|t| level_bezier_frame(&CURVE, t))));
    let swept = sweep::solid(m, mill, &along)?;
    kentron::boolean::difference(m, block, swept)
}
