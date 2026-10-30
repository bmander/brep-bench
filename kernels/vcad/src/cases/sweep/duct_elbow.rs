//! A hollow square duct, 18 × 18 with a 14 × 14 hole, its centreline the quarter of the circle of
//! radius 30 about the z axis from (30, 0, 0) to (0, 30, 0), its section in the plane through the
//! z axis, two sides horizontal.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchError, SketchProfile, SketchSegment};
use vcad_kernel::vcad_kernel_sweep::{CylindricalPath, SweepOptions};
use vcad_kernel::Solid;

const CENTRE_RADIUS: f64 = 30.0;
/// Half the outside's side and half the hole's.
const OUTSIDE: f64 = 9.0;
const HOLE: f64 = 7.0;
const TURN: f64 = 90.0;
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
    // from +x towards +y. It is sampled at vcad's own step, 0.5°.
    let arc = |from: f64, to: f64| {
        CylindricalPath::from_knots(CENTRE_RADIUS, vec![(from, 0.0), (to, 0.0)])
    };
    let duct = arc(0.0, TURN);
    let outside = Solid::sweep_cylindrical(square(OUTSIDE)?, &duct, SweepOptions::default())?;
    // a vcad sweep has one loop, so the hole is cut by its own sweep, run past both ends
    let hole = arc(-OVERRUN, TURN + OVERRUN);
    let cutter = Solid::sweep_cylindrical(square(HOLE)?, &hole, SweepOptions::default())?;
    Ok(outside.try_difference(&cutter)?)
}
