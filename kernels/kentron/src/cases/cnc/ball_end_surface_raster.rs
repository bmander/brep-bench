//! The plate from (0, 0, −20) to (100, 80, 0), its top finished by a ball-end mill of radius 3
//! (a half ball under a cylinder of radius 3 up to 30 above its tip), its axis upright, in 41
//! raster passes: pass j along y = 2j, the tip run straight from point to point through
//! (−5 + 5i, 2j, f) for i = 0 … 22, f(x, y) = −5 + 4 sin(πx/50) sin(πy/40), the even passes
//! along +x and the odd back along −x, each pass's last point joined straight to the next's first.
//! The tip plunges from (−5, 0, 5) to the first point and rises from the last to (−5, 80, 5). The
//! plate less everything the mill passes through.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one), so each pass is built as
//! elementary geometry gives it. The mill is everything within 3 of the part of its axis from
//! the ball's centre, 3 above the tip, to 27 above that, less the half ball over its top: and
//! its top never comes lower than z = 21, above the plate. Along a pass that part of the axis
//! stays in the plane y = 2j and passes through the region of it between the centre's path
//! (the tip's, 3 higher) and the same path 27 higher. So below its top the mill passes through
//! everything within 3 of that region: the region slid 3 to either side of its plane (a prism)
//! and everything within 3 of its boundary. Of the boundary only the centre's path reaches the
//! plate: a cylinder of radius 3 about each of its steps and a ball of radius 3 at each of its
//! points; the region's two upright ends are at x = −5 and x = 105, 2 clear of the plate, and
//! its top is above it. The plunge, the rise and the moves from one pass to the next are all at
//! x = −5 or x = 105 too, and cut nothing.
use crate::bench::{frame, p, v};
use crate::shared::geometry::{flat, segment};
use kentron::math::{Direction, Point};
use kentron::primitives::{Cuboid, Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use kentron::{boolean, sweep};
use std::f64::consts::PI;

const RADIUS: f64 = 3.0;
const SHANK: f64 = 27.0; // from the ball's centre, 3 above the tip, to 30 above it
const PASSES: usize = 41;

/// The surface the tip is run over.
fn f(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

/// What the mill passes through along the pass at `y`, below its top.
fn pass(m: &mut Model, y: f64) -> kentron::Result<Shape> {
    let centres: Vec<Point> = (0..=22)
        .map(|i| {
            let x = -5.0 + 5.0 * i as f64;
            p(x, y, f(x, y) + RADIUS)
        })
        .collect();
    // the region between the centre's path and the same path a shank higher, drawn 3 to one
    // side of the pass's plane and slid through to 3 on the other
    let low = centres.iter().map(|c| p(c.x, y - RADIUS, c.z));
    let high = centres.iter().rev().map(|c| p(c.x, y - RADIUS, c.z + SHANK));
    let outline: Vec<Point> = low.chain(high).collect();
    let mut edges = Vec::new();
    for k in 0..outline.len() {
        edges.push(segment(m, outline[k], outline[(k + 1) % outline.len()])?);
    }
    let region = flat(m, &edges)?;
    let mut swept = sweep::extrude(m, region, v(0.0, 2.0 * RADIUS, 0.0))?.shape;
    for step in centres.windows(2) {
        let t = step[1] - step[0];
        let tube = Cylinder::new(RADIUS, t.length()).with_frame(frame(step[0], Direction::new(t.x, t.y, t.z))).build(m)?;
        swept = boolean::union(m, swept, tube)?;
    }
    for &centre in &centres {
        let ball = Sphere::new(RADIUS).with_frame(frame(centre, Direction::Z)).build(m)?;
        swept = boolean::union(m, swept, ball)?;
    }
    Ok(swept)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -20.0), p(100.0, 80.0, 0.0)).build(m)?;
    // (The passes built at once, each apart from the others, and cut as one.)
    let passes = kentron::parallel::each(m, PASSES, |m, j| pass(m, 2.0 * j as f64))?;
    let swept = boolean::union_all(m, &passes)?;
    boolean::difference(m, plate, swept)
}
