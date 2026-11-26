//! `swept/box-screwed-inside-a-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: the rod of radius 5 about the line
//! through the origin along w, from the origin to 20w, less everything the unit cube
//! 2.5 ≤ p·u ≤ 3.5, |p·v| ≤ 0.5, 2 ≤ p·w ≤ 3 passes through as it is screwed about that line,
//! rising 4 a turn, from a turn of 0° to one of 720°: a sealed helical tunnel.
//!
//! Built as the original is, each frame it places (the world's, at a point) placed with its x
//! along u, its y along v and its z along w: the shared box and rod written out on those frames,
//! and the shared screw about w.
use std::f64::consts::PI;

use springrep::boolean::difference;
use springrep::math::{Point3, Vec3};
use springrep::primitive::{block, cylinder};
use springrep::sweep::{solid, Helix, Motion};
use springrep::topo::Brep;

use crate::shared::swept::deg;
use crate::shared::{e, frame};

const U: [f64; 3] = [2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0];
const V: [f64; 3] = [2.0 / 3.0, 1.0 / 3.0, -2.0 / 3.0];
const W: [f64; 3] = [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0];

/// The point the upright case puts at (x, y, z), here at x·u + y·v + z·w.
fn at(x: f64, y: f64, z: f64) -> [f64; 3] {
    [0, 1, 2].map(|i| x * U[i] + y * V[i] + z * W[i])
}

pub fn build() -> Result<Brep, String> {
    let bar = block(&frame(at(2.5, -0.5, 2.0), W, U)?, Vec3::new(1.0, 1.0, 1.0), "bar").map_err(e)?;
    let screw = Motion::Screw {
        screw: Helix { origin: Point3::ZERO, axis: Vec3::from_array(W), rise: 4.0 / (2.0 * PI), sign: 1.0 },
        from: 0.0,
        to: deg(720.0),
    };
    let swept = solid(&bar, &screw, "screw").map_err(e)?;
    let rod = cylinder(&frame([0.0, 0.0, 0.0], W, U)?, 5.0, 20.0, "rod").map_err(e)?;
    difference(&rod, &swept).map_err(e)
}
