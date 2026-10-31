//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre runs along the cubic Bézier curve in the top face through the
//! control points (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! solidcraft-kernel has no sweep of a solid, so the ball's sweep is built by elementary geometry:
//! every point within 3 of the curve, which (the curve turning no tighter than about 7.5) is the
//! ball's great disc square to the curve carried along it, with the ball at each end. Half of each
//! end ball lies inside the tube; the other half is the half ball on the tube's end disc, as in
//! `ball-swung-groove`. The tube and the half balls are cut from the block in turn.
//!
//! Nor does the kernel's `sweep` take a curve for its path, only lines and arcs: SolidCraft's own
//! Sweep follows a free-form path curve as the polyline its geometry crate draws through it
//! (`Seg2::polyline`, within 1e-2 of the curve), and so does this. A path of many short lines is
//! swept smoothly, with B-spline walls through stations along it.
//!
//! Where a half ball meets the B-spline tube's groove along its end disc's rim, the kernel's
//! boolean works for minutes and then finds no result (joined to the tube first, as in
//! `ball-swung-groove`, or whole balls, it fares no better).
use super::ball_swung_groove::half_ball;
use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_kernel::{PathSeg, sweep};

pub fn build() -> R<Body> {
    let block = block((-6.0, -6.0, -8.0), (36.0, 26.0, 0.0))?;
    // the curve, drawn in the top face (the xy plane at z = 0), followed as SolidCraft's Sweep
    // follows a free-form path
    let curve =
        Seg2::Cubic { p0: v2(0.0, 0.0), p1: v2(10.0, 0.0), p2: v2(20.0, 20.0), p3: v2(30.0, 20.0) };
    let points: Vec<Vec3> = curve.polyline(1e-2).iter().map(|p| Plane::XY.to_world(*p)).collect();
    let path: Vec<PathSeg> =
        points.windows(2).map(|w| PathSeg::Line { a: w[0], b: w[1] }).collect();
    // the disc where the curve starts, square to it: the curve leaves (0, 0, 0) along +x
    let start = Plane { origin: Vec3::ZERO, x: Vec3::Y, y: Vec3::Z };
    let mut grooved = cut(&block, &sweep(&start, &disc((0.0, 0.0), 3.0), &path)?)?;
    // the half balls beyond the tube's ends: the curve leaves its first end along +x and reaches
    // its last along +x
    for (end, outward) in [(v3(0.0, 0.0, 0.0), -Vec3::X), (v3(30.0, 20.0, 0.0), Vec3::X)] {
        grooved = cut(&grooved, &half_ball(end, outward, 3.0)?)?;
    }
    Ok(grooved)
}
