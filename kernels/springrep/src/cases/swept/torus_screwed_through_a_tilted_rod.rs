//! `swept/torus-screwed-through-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: the rod of radius 5 about the line
//! through the origin along w, from the origin to 20w, less everything a torus wheel (radii 6 and 2
//! about 11.5u, its axis cos h·v − sin h·w, h = atan(2π·5.5/30)) passes through as it is screwed
//! about that line, rising 30 a turn, from −108° to 348°.
//!
//! Built as the original is, each frame and direction it places at (x, y, z) placed at
//! x·u + y·v + z·w: the shared rod and reach cylinders written out on frames along w, the shared
//! screw about w.
use std::f64::consts::PI;

use springrep::boolean::{difference, intersection};
use springrep::math::{Point3, Vec3};
use springrep::primitive::{cylinder, torus};
use springrep::sweep::{solid, Helix, Motion};
use springrep::topo::Brep;

use crate::shared::swept::deg;
use crate::shared::{e, frame};

const U: [f64; 3] = [2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0];
const V: [f64; 3] = [2.0 / 3.0, 1.0 / 3.0, -2.0 / 3.0];
const W: [f64; 3] = [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0];

/// The point (or direction) the upright case puts at (x, y, z), here at x·u + y·v + z·w.
fn at(x: f64, y: f64, z: f64) -> [f64; 3] {
    [0, 1, 2].map(|i| x * U[i] + y * V[i] + z * W[i])
}

pub fn build() -> Result<Brep, String> {
    let h = (2.0 * PI * 5.5 / 30.0).atan();
    let axis = frame(at(11.5, 0.0, 0.0), at(0.0, h.cos(), -h.sin()), U)?;
    let wheel = torus(&axis, 6.0, 2.0, "wheel").map_err(e)?;
    // No sweep of the whole tool: a screw keeps each point's distance from its axis, so only the
    // tool within 5.5 of it reaches the rod, and only that part is swept.
    let reach = cylinder(&frame(at(0.0, 0.0, -100.0), W, U)?, 5.5, 200.0, "reach").map_err(e)?;
    let rim = intersection(&wheel, &reach).map_err(e)?;
    let screw = Motion::Screw {
        screw: Helix { origin: Point3::ZERO, axis: Vec3::from_array(W), rise: 30.0 / (2.0 * PI), sign: 1.0 },
        from: deg(-108.0),
        to: deg(348.0),
    };
    let swept = solid(&rim, &screw, "screw").map_err(e)?;
    let rod = cylinder(&frame([0.0, 0.0, 0.0], W, U)?, 5.0, 20.0, "rod").map_err(e)?;
    difference(&rod, &swept).map_err(e)
}
