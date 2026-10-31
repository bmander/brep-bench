use crate::solids::*;
use solidcraft_kernel::{PathSeg, SweepControl, sweep_with};
use std::f64::consts::TAU;

/// SolidCraft draws a section turning steadily along a straight run as a sweep with a twist,
/// which grows in proportion to the length run, about the path. The rectangle 40 × 0.4 at
/// z = 0, swept up the z axis to z = 60, turns a whole turn right-handed about the way it
/// travels, 6° per unit, its sides straight: each wall is swept by one of them, and each corner
/// runs along a helix rising 60 in a turn.
pub fn build() -> R<Body> {
    let up = PathSeg::Line { a: Vec3::ZERO, b: v3(0.0, 0.0, 60.0) };
    let ctl = SweepControl { twist: TAU, ..SweepControl::default() };
    sweep_with(&Plane::XY, &rect(-20.0, -0.2, 20.0, 0.2), &[up], &ctl)
}
