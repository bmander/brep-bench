//! A bar of square section, 20 × 20, bent along the planar cubic Bézier curve with control points
//! (0, 0, 0), (16, 12, 0), (18, 26, 0) and (12, 34, 0), two of its sides horizontal, its ends flat.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::vcad_kernel_sweep::SweepOptions;
use vcad_kernel::Solid;
use vcad_kernel_nurbs::BSplineCurve;

/// Half the square's side.
const HALF: f64 = 10.0;
const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [16.0, 12.0, 0.0], [18.0, 26.0, 0.0], [12.0, 34.0, 0.0]];

pub fn build() -> Result<Solid, crate::Refusal> {
    // vcad-kernel-nurbs's B-spline implements the sweep's `Curve3d`; a cubic Bézier is the clamped
    // cubic B-spline on its four control points, knots (0, 0, 0, 0, 1, 1, 1, 1)
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let path = BSplineCurve::new(points, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
    // the sweep reads the section in sketch coordinates about the path, its x along the frame's
    // normal and its y along the binormal; on a curve in the xy plane the normal lies in that plane
    // and the binormal is vertical, so a square about the origin keeps two sides horizontal
    let corners = [(HALF, -HALF), (HALF, HALF), (-HALF, HALF), (-HALF, -HALF)];
    let corner = |i: usize| Point2::new(corners[i % 4].0, corners[i % 4].1);
    let sides = (0..4).map(|i| SketchSegment::Line { start: corner(i), end: corner(i + 1) });
    let section = SketchProfile::new(Point3::origin(), Vec3::x(), Vec3::y(), sides.collect())?;
    // vcad's sweep is faceted: the path sampled at SEGMENTS places along it
    let options = SweepOptions { path_segments: SEGMENTS, ..SweepOptions::default() };
    Ok(Solid::sweep(section, &path, options)?)
}
