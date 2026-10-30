//! A hollow square tube tapering along the y axis from y = 0 (24 × 24 outside, a 20 × 20 hole) to
//! y = 40 (12 × 12, an 8 × 8 hole), its sections squares about the y axis, sides along x and z.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchError, SketchProfile, SketchSegment};
use vcad_kernel::vcad_kernel_sweep::LoftOptions;
use vcad_kernel::Solid;

const LENGTH: f64 = 40.0;
/// Half the outside's side and half the hole's, at y = 0 and at y = 40.
const OUTSIDE: (f64, f64) = (12.0, 6.0);
const HOLE: (f64, f64) = (10.0, 4.0);
/// How far the hole's cutter runs past each end of the tube.
const OVERRUN: f64 = 1.0;

/// The square of half-side `half` about the y axis in the plane at `y`, sides along x and z.
fn square(y: f64, half: f64) -> Result<SketchProfile, SketchError> {
    let corners = [(half, -half), (half, half), (-half, half), (-half, -half)];
    let corner = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
    let sides = (0..4).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    SketchProfile::new(Point3::new(0.0, y, 0.0), Vec3::x(), Vec3::z(), sides.collect())
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's ruled loft joins each corner of one section to the same corner of the next by a
    // straight line, so the long faces are the flat trapezoids between them
    let outside = Solid::loft(
        &[square(0.0, OUTSIDE.0)?, square(LENGTH, OUTSIDE.1)?],
        LoftOptions::default(),
    )?;
    // a loft has one loop, so the hole is cut by its own loft, run past both ends: its half-side
    // keeps running linearly with y beyond them
    let half = |y: f64| HOLE.0 + (HOLE.1 - HOLE.0) * y / LENGTH;
    let (near, far) = (-OVERRUN, LENGTH + OVERRUN);
    let cutter =
        Solid::loft(&[square(near, half(near))?, square(far, half(far))?], LoftOptions::default())?;
    Ok(outside.try_difference(&cutter)?)
}
