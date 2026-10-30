//! A pipe of circular section, radius 8, swept along the cubic Bézier curve, not in a plane, with
//! control points (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16), its ends flat.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::SweepOptions;
use vcad_kernel::Solid;
use vcad_kernel_nurbs::BSplineCurve;

const RADIUS: f64 = 8.0;
const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [12.0, 16.0, 0.0], [26.0, 8.0, 0.0], [24.0, 0.0, 16.0]];
/// The section's circle is cut into this many arcs, each into SEGMENTS / ARCS lines.
const ARCS: u32 = 4;

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad-kernel-nurbs's B-spline implements the sweep's `Curve3d`; a cubic Bézier is the clamped
    // cubic B-spline on its four control points, knots (0, 0, 0, 0, 1, 1, 1, 1)
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let path = BSplineCurve::new(points, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
    // the sweep reads the section in sketch coordinates about the path: a circle about the origin
    let section = SketchProfile::circle(Point3::origin(), Vec3::z(), RADIUS, ARCS);
    // vcad's sweep is faceted: the circle cut into SEGMENTS lines, as a primitive's is, and the
    // path sampled at SEGMENTS places along it
    let options = SweepOptions {
        path_segments: SEGMENTS,
        arc_segments: SEGMENTS / ARCS,
        ..SweepOptions::default()
    };
    Ok(Solid::sweep(section, &path, options)?)
}
