//! The plate from (0, 0, −20) to (100, 80, 0), its top finished by a ball-end mill of radius 3
//! (a half ball under a cylinder of radius 3 up to 30 above its tip), its axis upright, in 41
//! raster passes: pass j along y = 2j, the tip run straight from point to point through
//! (−5 + 5i, 2j, f) for i = 0 … 22, f(x, y) = −5 + 4 sin(πx/50) sin(πy/40), the even passes
//! along +x and the odd back along −x, each pass's last point joined straight to the next's first.
//! The tip plunges from (−5, 0, 5) to the first point and rises from the last to (−5, 80, 5). The
//! plate less everything the mill passes through.
//!
//! kentron has no sweep of a solid (`sweep::pipe` refuses one), so each straight move is built as
//! elementary geometry gives it. The mill is its ball and its shank, the cylinder from the ball's
//! centre up. The ball slid along a line passes through itself at either end and the cylinder of
//! radius 3 about its centre's path between. The shank, convex, passes through itself at the
//! move's end and what its trailing faces cross as they slide: a point it covers at some instant
//! and not at the end leaves it, the last time it is inside, through a face the slide points away
//! from. Those are the half of its side behind the upright lines where the slide runs along it
//! (square to the slide's level part), slid along the move, and its bottom (inside the ball all
//! the way, so inside the ball's sweep) or its top (above the plate). The mill at the start is
//! the last move's at its end. A move straight up or down slides the mill along its own axis:
//! it passes through itself at both ends and the ball's cylinder between, the shank above. The
//! mill at the start, its tip at z = 5, is above the plate.
use crate::bench::{frame, p, v};
use crate::shared::swept::{cut_each, solids};
use kentron::geometry::surface::{Cylinder as CylinderSurface, Surface};
use kentron::math::{Direction, Frame, Point};
use kentron::primitives::{Cuboid, Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use kentron::{make, sweep};
use std::f64::consts::{FRAC_PI_2, PI};

const RADIUS: f64 = 3.0;
const SHANK: f64 = 27.0; // from the ball's centre, 3 above the tip, to 30 above it
const PASSES: usize = 41;

/// The surface the tip is run over.
fn f(x: f64, y: f64) -> f64 {
    -5.0 + 4.0 * (PI * x / 50.0).sin() * (PI * y / 40.0).sin()
}

/// The tip's positions in order.
fn program() -> Vec<Point> {
    let mut tip = vec![p(-5.0, 0.0, 5.0)];
    for j in 0..PASSES {
        let y = 2.0 * j as f64;
        for k in 0..23 {
            let i = if j % 2 == 0 { k } else { 22 - k };
            let x = -5.0 + 5.0 * i as f64;
            tip.push(p(x, y, f(x, y)));
        }
    }
    let last = *tip.last().unwrap();
    tip.push(p(last.x, last.y, 5.0));
    tip
}

/// What the mill passes through as its tip moves straight from `a` to `b`, less the mill at `a`.
fn slide(m: &mut Model, a: Point, b: Point, cuts: &mut Vec<Shape>) -> kentron::Result<()> {
    let up = v(0.0, 0.0, RADIUS);
    let (from, to) = (a + up, b + up);
    let t = to - from;
    let ball = Sphere::new(RADIUS).with_frame(frame(to, Direction::Z)).build(m)?;
    let shank = Cylinder::new(RADIUS, SHANK).with_frame(frame(to, Direction::Z)).build(m)?;
    let along = Cylinder::new(RADIUS, t.length()).with_frame(frame(from, Direction::new(t.x, t.y, t.z))).build(m)?;
    cuts.extend([along, ball, shank]);
    if t.x.hypot(t.y) > 0.0 {
        // the trailing half of the shank's side, from where its normal is square to the slide
        let side = Surface::Cylinder(CylinderSurface::new(Frame::new(from, Direction::Z, Direction::new(t.x, t.y, 0.0)), RADIUS));
        let half = make::face_on_domain(m, side, FRAC_PI_2, 3.0 * FRAC_PI_2, 0.0, SHANK)?;
        let slid = sweep::extrude(m, half, t)?.shape;
        cuts.extend(solids(m, slid));
    }
    Ok(())
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -20.0), p(100.0, 80.0, 0.0)).build(m)?;
    let tip = program();
    let mut cuts = Vec::new();
    for pair in tip.windows(2) {
        slide(m, pair[0], pair[1], &mut cuts)?;
    }
    cut_each(m, plate, &cuts)
}
