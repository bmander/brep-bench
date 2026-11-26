//! A block, the box from (0, −5, 0) to (20, 5, 10), V-grooved across its top by a wheel with a
//! sharp rim: the solid of revolution, about the line through (−8, 0, 14) along y, of the
//! triangle in the plane x = −8 with corners (y, z) = (0, 8) and (±3 tan 30°, 11), slid without
//! turning along x by 36. Everything the wheel passes through, cut.
//!
//! The wheel is the triangle revolved; its sweep is kentron's sweep of a solid (`sweep::solid`,
//! one slide), cut from the block.
use crate::bench::{p, v};
use crate::shared::sweep::polyline;
use kentron::make;
use kentron::math::{Axis, Direction};
use kentron::primitives::Cuboid;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 10.0)).build(m)?;
    let half = 3.0 * (PI / 6.0).tan();
    let outline = polyline(m, &[p(-8.0, 0.0, 8.0), p(-8.0, half, 11.0), p(-8.0, -half, 11.0)], true)?;
    let triangle = make::planar_face(m, outline)?;
    let wheel = sweep::revolve(m, triangle, Axis::new(p(-8.0, 0.0, 14.0), Direction::Y), TAU)?.shape;
    let swept = sweep::solid(m, wheel, &RigidMotion::Slide { by: v(36.0, 0.0, 0.0) })?;
    kentron::boolean::difference(m, block, swept)
}
