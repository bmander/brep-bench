//! A pulley on the z axis, symmetric about z = 0: a hub of radius 12 from z = −14 to 14, a rim of
//! radius 28 from z = −9 to 9, a bore of radius 6 through, and a V groove round the rim, the
//! triangle (22, 0), (29, −5), (29, 5) in xz turned a full turn and taken away.
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const BORE: f64 = 6.0;
const HUB_RADIUS: f64 = 12.0;
const HUB_HALF: f64 = 14.0;
const RIM_RADIUS: f64 = 28.0;
const RIM_HALF: f64 = 9.0;
const GROOVE: [(f64, f64); 3] = [(22.0, 0.0), (29.0, -5.0), (29.0, 5.0)];

/// The closed polygon through `corners` in the xz plane, its x along x and its y along z.
fn section(corners: &[(f64, f64)]) -> Result<SketchProfile, crate::Refusal> {
    let n = corners.len();
    let corner = |i: usize| Point2::new(corners[i % n].0, corners[i % n].1);
    let sides = (0..n).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    Ok(SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::z(), sides.collect())?)
}

pub fn build() -> Result<Solid, crate::Refusal> {
    // the half section of hub, rim and bore, from the bore out
    let body = section(&[
        (BORE, -HUB_HALF),
        (HUB_RADIUS, -HUB_HALF),
        (HUB_RADIUS, -RIM_HALF),
        (RIM_RADIUS, -RIM_HALF),
        (RIM_RADIUS, RIM_HALF),
        (HUB_RADIUS, RIM_HALF),
        (HUB_RADIUS, HUB_HALF),
        (BORE, HUB_HALF),
    ])?;
    let pulley = Solid::revolve(body, Point3::origin(), Vec3::z(), 360.0)?;
    // the groove's triangle reaches 1 past the rim's face, so the cut opens cleanly
    let groove = Solid::revolve(section(&GROOVE)?, Point3::origin(), Vec3::z(), 360.0)?;
    Ok(pulley.try_difference(&groove)?)
}
