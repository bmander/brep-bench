//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre runs along the cubic Bézier curve in the top face with control
//! points (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! vcad has no sweep of a solid (`Solid::sweep` moves a sketch profile), so the ball's sweep is
//! built from what it is by elementary geometry: every point within 3 of the curve. Where the curve
//! turns no tighter than the ball's radius (its tightest turn is about 7.5), that is the tube the
//! ball's great disc square to the curve fills as it runs, vcad's sweep of the disc along the
//! curve, with a ball at each end: half of each lies inside the tube.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::SketchProfile;
use vcad_kernel::vcad_kernel_sweep::SweepOptions;
use vcad_kernel::Solid;
use vcad_kernel_nurbs::BSplineCurve;

const BLOCK_MIN: [f64; 3] = [-6.0, -6.0, -8.0];
const BLOCK_MAX: [f64; 3] = [36.0, 26.0, 0.0];
const BALL_RADIUS: f64 = 3.0;
const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [20.0, 20.0, 0.0], [30.0, 20.0, 0.0]];
/// The disc's circle is cut into this many arcs, each into SEGMENTS / ARCS lines.
const ARCS: u32 = 4;

pub fn build() -> Result<Solid, crate::Refusal> {
    let [x0, y0, z0] = BLOCK_MIN;
    let [x1, y1, z1] = BLOCK_MAX;
    let block = Solid::cube(x1 - x0, y1 - y0, z1 - z0).translate(x0, y0, z0);
    // vcad-kernel-nurbs's B-spline implements the sweep's `Curve3d`; a cubic Bézier is the clamped
    // cubic B-spline on its four control points, knots (0, 0, 0, 0, 1, 1, 1, 1)
    let points = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    let path = BSplineCurve::new(points, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
    // the sweep reads the disc in sketch coordinates about the path: a circle about the origin.
    // vcad's sweep is faceted: the circle cut into SEGMENTS lines, as a primitive's is, and the
    // path sampled at SEGMENTS places along it
    let disc = SketchProfile::circle(Point3::origin(), Vec3::z(), BALL_RADIUS, ARCS);
    let options = SweepOptions {
        path_segments: SEGMENTS,
        arc_segments: SEGMENTS / ARCS,
        ..SweepOptions::default()
    };
    let tube = Solid::sweep(disc, &path, options)?;
    let ball = |[x, y, z]: [f64; 3]| Solid::sphere(BALL_RADIUS, SEGMENTS).translate(x, y, z);
    // the tube and the ball at each end, each cut in turn
    let grooved = block.try_difference(&tube)?;
    let grooved = grooved.try_difference(&ball(CONTROL[0]))?;
    Ok(grooved.try_difference(&ball(CONTROL[3]))?)
}
