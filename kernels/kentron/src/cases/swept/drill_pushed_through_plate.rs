//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a hole bored through it by a drill about
//! the vertical line through (10, 0): a cylinder of radius 2 from z = 6 + 2/tan 59° up to 14, ended
//! below in a cone of 118° whose point is (10, 0, 6), slid without turning straight down its axis
//! by 20. Everything the drill passes through, cut.
//!
//! The drill is its half section revolved about its axis; its sweep is kentron's sweep of a solid
//! (`sweep::solid`, one slide), cut from the plate.
use crate::bench::{p, v};
use crate::shared::parts::xz_plane;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cuboid;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Start, To};
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    // Where the cone meets the cylinder.
    let rim = 6.0 + 2.0 / 59f64.to_radians().tan();
    let half = [Start(10.0, 6.0), To(12.0, rim), To(12.0, 14.0), To(10.0, 14.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let drill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let swept = sweep::solid(m, drill, &RigidMotion::Slide { by: v(0.0, 0.0, -20.0) })?;
    kentron::boolean::difference(m, plate, swept)
}
