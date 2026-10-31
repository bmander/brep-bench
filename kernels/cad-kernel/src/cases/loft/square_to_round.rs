//! A square-to-round transition: the square ±20 at z = 0 to the circle of radius 15 about the z
//! axis at z = 30, each side joined by a flat triangle to the circle's point nearest it, each
//! corner by a piece of an oblique cone to the quarter of the circle facing it.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

const HALF: f64 = 20.;
const RADIUS: f64 = 15.;
const HEIGHT: f64 = 30.;
/// The square's corners, counter-clockwise about z from (20, −20).
const CORNERS: [[f64; 2]; 4] = [[HALF, -HALF], [HALF, HALF], [-HALF, HALF], [-HALF, -HALF]];
/// The directions from the axis to the circle's four points: corner `k`'s quarter of the circle
/// runs from the one at `k` to the one at `k + 1`, and the side after corner `k` meets the circle
/// at the one at `k + 1`.
const TOWARD: [[f64; 2]; 4] = [[0., -1.], [1., 0.], [0., 1.], [-1., 0.]];

/// The point a fraction `t` of the way up the straight line from `a` in the floor to `b` in the
/// top.
fn up(a: [f64; 2], b: [f64; 2], t: f64) -> V {
    [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), HEIGHT * t]
}

pub fn build() -> Result<Brep, String> {
    // A loft from one section to the other is a face family over the fraction t of the way up, one
    // piece with its six stations at the Gauss–Lobatto nodes. Every wall's lines run straight from
    // the foot to the top, so each number of the section is linear in t and the family is exact.
    let at = vec![NODES.map(|x| (1. + x) / 2.)];
    let stations = Arc::new(Stations::new(vec![[0., 1.]], at)?);
    let mut walls = Vec::new();
    for k in 0..4 {
        let (c, next) = (CORNERS[k], CORNERS[(k + 1) % 4]);
        let (from, to) = (TOWARD[k], TOWARD[(k + 1) % 4]);
        // The cone at corner k: at t the section's corner is the arc of radius 15t about the point
        // (1 − t) of the way along the line from the axis to the corner, a quarter turn from
        // `from` to `to`, so each of its points lies on the line from the corner to the point of
        // the circle in that direction. It is nothing at the foot: an edge vanished there.
        let cone = stations.at.iter().flatten().map(|&t| {
            let r = RADIUS * t;
            let centre = up(c, [0., 0.], t);
            let (r0, r90) = ([r * from[0], r * from[1], 0.], [r * to[0], r * to[1], 0.]);
            [centre, r0, r90].concat().into_iter().chain([FRAC_PI_2]).collect()
        }).collect();
        let sheet = Sheet::new(Run::Round, stations.clone(), cone, false)?;
        walls.push(Wall { sheet: Arc::new(sheet), name: "cone".into() });
        // The triangle on the side from corner k to the next: at t the segment between the lines
        // from those corners to the circle's point `to`, vanished at the top.
        let point = [RADIUS * to[0], RADIUS * to[1]];
        let side = stations.at.iter().flatten().map(|&t| {
            let (a, b) = (up(c, point, t), up(next, point, t));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), side, false)?;
        walls.push(Wall { sheet: Arc::new(sheet), name: "triangle".into() });
    }
    // an edge no longer than a billionth at an end has vanished there
    family(&[walls], 1e-9, None)
}
