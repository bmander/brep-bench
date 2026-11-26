//! The plate from (0, 0, −20) to (100, 80, 0), its top finished by a ball-end mill of radius 3
//! (a half ball under a cylinder of radius 3 up to 30 above its tip), its axis upright, in 41
//! raster passes: pass j along y = 2j, the tip run straight from point to point through
//! (−5 + 5i, 2j, f) for i = 0 … 22, f(x, y) = −5 + 4 sin(πx/50) sin(πy/40), the even passes
//! along +x and the odd back along −x, each pass's last point joined straight to the next's first.
//! The tip plunges from (−5, 0, 5) to the first point and rises from the last to (−5, 80, 5). The
//! plate less everything the mill passes through.
//!
//! The mill is its half section revolved about its axis. Each straight move of each pass is
//! kentron's sweep of a solid of the mill (`sweep::solid`, a slide); a pass is the union of its
//! moves, the passes are built at once, each apart from the others, and cut as one. The plunge,
//! the rise and the moves from one pass to the next are all at x = −5 or x = 105, the mill 2
//! clear of the plate, and cut nothing, so they are not swept.
use crate::bench::p;
use crate::shared::cnc::{line, swept, upright};
use kentron::boolean;
use kentron::math::Point;
use kentron::primitives::Cuboid;
use kentron::sketch::Step::{Arc, Heading, Start, To};
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

const PASSES: usize = 41;

/// The surface the tip is run over.
fn f(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

/// What the mill passes through along the pass at `y`: its moves from point to point, each swept,
/// united.
fn pass(m: &mut Model, y: f64) -> kentron::Result<Shape> {
    let points: Vec<Point> = (0..=22)
        .map(|i| {
            let x = -5.0 + 5.0 * i as f64;
            p(x, y, f(x, y))
        })
        .collect();
    let moves: Vec<_> = points.windows(2).map(|step| line(step[0], step[1])).collect();
    // The mill: the ball's quarter circle from the tip up to its equator, 3 above, and the side
    // up to 30 above the tip.
    let half = [Start(0.0, 0.0), Heading(0.0), Arc { radius: 3.0, angle: 90.0 }, To(3.0, 30.0), To(0.0, 30.0)];
    let sweeps = swept(m, |m| upright(m, &half), &moves)?;
    boolean::union_all(m, &sweeps)
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -20.0), p(100.0, 80.0, 0.0)).build(m)?;
    // (The passes built at once, each apart from the others, and cut as one.)
    let passes = kentron::parallel::each(m, PASSES, |m, j| pass(m, 2.0 * j as f64))?;
    let swept = boolean::union_all(m, &passes)?;
    boolean::difference(m, plate, swept)
}
