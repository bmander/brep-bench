use crate::solids::*;
use solidcraft_kernel::{PathSeg, SweepControl, sweep_with};
use std::f64::consts::PI;

/// SolidCraft draws a section turning and shrinking steadily along a straight run as a sweep
/// with a twist and an end scale: both grow in proportion to the length run, about the path.
/// The square of side 30 at z = 0, swept up the z axis to z = 40, turns half a turn right-handed
/// about the way it travels and ends a third of its size, so at a height z it is the square of
/// side 30 − z/2 turned 4.5° per unit, its sides straight: each wall is swept by one of them.
pub fn build() -> R<Body> {
    let up = PathSeg::Line { a: Vec3::ZERO, b: v3(0.0, 0.0, 40.0) };
    let ctl = SweepControl { twist: PI, scale: 10.0 / 30.0, ..SweepControl::default() };
    sweep_with(&Plane::XY, &rect(-15.0, -15.0, 15.0, 15.0), &[up], &ctl)
}
