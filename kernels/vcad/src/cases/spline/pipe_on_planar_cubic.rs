//! A pipe of circular section, radius 20, swept along the planar cubic Bézier curve with control
//! points (0, 0, 0), (12, −16, 0), (32, −16, 0) and (44, 0, 0), its ends flat.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::SweepOptions;
use vcad_kernel::Solid;
use vcad_kernel_nurbs::BSplineCurve;

const RADIUS: f64 = 20.0;
const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [12.0, -16.0, 0.0], [32.0, -16.0, 0.0], [44.0, 0.0, 0.0]];
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
