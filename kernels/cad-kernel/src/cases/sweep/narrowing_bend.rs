//! A hollow square duct round the quarter arc of radius 30 about z, half-width 9 → 6, wall 2.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

const BEND_RADIUS: f64 = 30.;
const HALF_START: f64 = 9.;
const HALF_END: f64 = 6.;
const WALL: f64 = 2.;
const TURN: f64 = FRAC_PI_2;
/// How many pieces the quarter turn is read in, six stations each: the walls are the section's
/// sides at each station and the degree-five polynomial through them between, which misses the
/// corners' spirals over a piece of a sixteenth of the turn by under a millionth of a micron.
const PIECES: usize = 16;

/// The section at `theta` from the xz plane: a square loop of half-width `half` (less `inset`)
/// about the arc's point, in the vertical plane through the z axis there, as its four corners.
fn square(theta: f64, inset: f64) -> Vec<V> {
    let half = HALF_START + (HALF_END - HALF_START) * theta / TURN - inset;
    let (s, c) = theta.sin_cos();
    let at = |rho: f64, z: f64| [rho * c, rho * s, z];
    let r = BEND_RADIUS;
    vec![at(r - half, -half), at(r + half, -half), at(r + half, half), at(r - half, half)]
}

/// The walls one loop of the section sweeps: each side a line whose two ends the station moves,
/// its numbers read at every station.
fn walls(stations: &Arc<Stations>, inset: f64, name: &str) -> Result<Vec<Wall>, String> {
    (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&theta| {
            let corners = square(theta, inset);
            let (a, b) = (corners[k], corners[(k + 1) % 4]);
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: name.into() })
    }).collect()
}

pub fn build() -> Result<Brep, String> {
    // a section that changes as it runs is a face family: the turn in pieces, each with its six
    // stations at the Gauss–Lobatto nodes
    let step = TURN / PIECES as f64;
    let spans: Vec<[f64; 2]> =
        (0..PIECES).map(|j| [step * j as f64, step * (j + 1) as f64]).collect();
    let at = spans.iter().map(|&[a, b]| NODES.map(|t| (a + b) / 2. + t * (b - a) / 2.)).collect();
    let stations = Arc::new(Stations::new(spans, at)?);
    // the outer loop first, then its hole; no edge vanishes at either end
    let loops = [walls(&stations, 0., "outside")?, walls(&stations, WALL, "inside")?];
    family(&loops, 0., None)
}
