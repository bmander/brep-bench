//! The solid a square bar fills as it is swung a quarter turn about an axis it faces with an
//! edge: the prism from z = 0 to z = 3 on the square with corners (4, 0), (6, 2), (8, 0) and
//! (6, −2), turned rigidly about the z axis by 90°.
//!
//! The bar is the square extruded; its sweep is kentron's sweep of a solid (`sweep::solid`, one
//! turn).
use crate::bench::{p, v};
use crate::shared::loft::flat;
use kentron::math::Axis;
use kentron::sweep::{self, RigidMotion};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let square = flat(m, &[p(4.0, 0.0, 0.0), p(6.0, -2.0, 0.0), p(8.0, 0.0, 0.0), p(6.0, 2.0, 0.0)])?;
    let bar = sweep::extrude(m, square, v(0.0, 0.0, 3.0))?.shape;
    sweep::solid(m, bar, &RigidMotion::Turn { axis: Axis::Z, from: 0.0, to: FRAC_PI_2 })
}
