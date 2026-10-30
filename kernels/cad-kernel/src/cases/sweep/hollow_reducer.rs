//! A hollow square tube on y from 0 to 40, outside 24 → 12 across, hole 20 → 8, linear in y.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const LENGTH: f64 = 40.;
const HALF_START: f64 = 12.;
const HALF_END: f64 = 6.;
const WALL: f64 = 2.;

/// The section at `y`: a square loop of half-width `half` (less `inset`) about the y axis, its
/// sides along x and z, as its four corners.
fn square(y: f64, inset: f64) -> Vec<V> {
    let half = HALF_START + (HALF_END - HALF_START) * y / LENGTH - inset;
    vec![[-half, y, -half], [half, y, -half], [half, y, half], [-half, y, half]]
}

/// The walls one loop of the section sweeps: each side a line whose two ends the station moves,
/// its numbers read at every station.
fn walls(stations: &Arc<Stations>, inset: f64, name: &str) -> Result<Vec<Wall>, String> {
    (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&y| {
            let corners = square(y, inset);
            let (a, b) = (corners[k], corners[(k + 1) % 4]);
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: name.into() })
    }).collect()
}

pub fn build() -> Result<Brep, String> {
    // a loft from one section to the other is a face family running over y, one piece with its
    // six stations at the Gauss–Lobatto nodes: every corner straight, so every side's wall is
    // the plane the kernel finds it lies on
    let at = vec![NODES.map(|t| LENGTH / 2. * (1. + t))];
    let stations = Arc::new(Stations::new(vec![[0., LENGTH]], at)?);
    let loops = [walls(&stations, 0., "outside")?, walls(&stations, WALL, "inside")?];
    family(&loops, 0., None)
}
