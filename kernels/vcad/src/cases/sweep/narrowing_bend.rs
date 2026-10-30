//! A hollow square duct narrowing round a quarter turn: its centreline the quarter of the circle
//! of radius 30 about the z axis from (30, 0, 0) to (0, 30, 0), its section in the plane through
//! the z axis, a square of half-width h = 9 − 3·θ/90° with a square hole of half-width h − 2.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchError, SketchProfile, SketchSegment};
use vcad_kernel::vcad_kernel_sweep::{CylindricalPath, SweepOptions};
use vcad_kernel::Solid;

const CENTRE_RADIUS: f64 = 30.0;
const TURN: f64 = 90.0;
/// The outside's half-width at the start and at the end; the wall's thickness.
const OUTSIDE: (f64, f64) = (9.0, 6.0);
const WALL: f64 = 2.0;
/// How far the hole's cutter runs past each end of the duct, in degrees.
const OVERRUN: f64 = 1.0;

/// The square of half-side `half` about the sketch's origin.
fn square(half: f64) -> Result<SketchProfile, SketchError> {
    let corners = [(half, -half), (half, half), (-half, half), (-half, -half)];
    let corner = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
    let sides = (0..4).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::y(), sides.collect())
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's cylindrical (cam-track) path holds the section in the plane through the z axis, its x
    // out from the axis and its y along z: knots (angle in degrees, height), here level, turning
    // from +x towards +y. It is sampled at vcad's own step, 0.5°, evenly in angle, so the sweep's
    // scale, linear along its samples, is linear in the angle turned.
    let arc = |from: f64, to: f64| {
        CylindricalPath::from_knots(CENTRE_RADIUS, vec![(from, 0.0), (to, 0.0)])
    };
    let scaled =
        |from: f64, to: f64| SweepOptions { scale_end: to / from, ..SweepOptions::default() };
    let outside = Solid::sweep_cylindrical(
        square(OUTSIDE.0)?,
        &arc(0.0, TURN),
        scaled(OUTSIDE.0, OUTSIDE.1),
    )?;
    // a vcad sweep has one loop, so the hole is cut by its own sweep, run past both ends: its
    // half-width keeps running linearly with the angle beyond them
    let hole = |angle: f64| OUTSIDE.0 - WALL + (OUTSIDE.1 - OUTSIDE.0) * angle / TURN;
    let (first, last) = (-OVERRUN, TURN + OVERRUN);
    let cutter = Solid::sweep_cylindrical(
        square(hole(first))?,
        &arc(first, last),
        scaled(hole(first), hole(last)),
    )?;
    Ok(outside.try_difference(&cutter)?)
}
