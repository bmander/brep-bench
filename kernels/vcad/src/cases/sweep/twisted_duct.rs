//! A solid duct along the y axis from the square A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10),
//! D (10, 0, 10) at y = 0 to the diamond A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12),
//! D′ (12, 40, 0) at y = 40, each corner running straight to its primed one, the sides ruled.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchError, SketchProfile, SketchSegment};
use vcad_kernel::vcad_kernel_sweep::LoftOptions;
use vcad_kernel::Solid;

const LENGTH: f64 = 40.0;
/// The corners A, B, C, D of each end in (x, z).
const SQUARE: [(f64, f64); 4] = [(10.0, -10.0), (-10.0, -10.0), (-10.0, 10.0), (10.0, 10.0)];
const DIAMOND: [(f64, f64); 4] = [(0.0, -12.0), (-12.0, 0.0), (0.0, 12.0), (12.0, 0.0)];

/// The quadrilateral through `corners` (x, z) in the plane at `y`.
fn section(y: f64, corners: [(f64, f64); 4]) -> Result<SketchProfile, SketchError> {
    let corner = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
    let sides = (0..4).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    SketchProfile::new(Point3::new(0.0, y, 0.0), Vec3::x(), Vec3::z(), sides.collect())
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad's ruled loft joins each corner of one section to the same corner of the next, A to A′
    // and so round, and each side between two such lines is one face (vcad gives it the plane
    // through three of its corners, though here its fourth is off that plane)
    let ends = [section(0.0, SQUARE)?, section(LENGTH, DIAMOND)?];
    Ok(Solid::loft(&ends, LoftOptions::default())?)
}
