//! A duct on y from the square (±10, 0, ±10) to a diamond 12 off the axis at y = 40, ruled.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const LENGTH: f64 = 40.;
// the corners A, B, C, D of each end
const SQUARE: [V; 4] = [[10., 0., -10.], [-10., 0., -10.], [-10., 0., 10.], [10., 0., 10.]];
const DIAMOND: [V; 4] =
    [[0., LENGTH, -12.], [-12., LENGTH, 0.], [0., LENGTH, 12.], [12., LENGTH, 0.]];

/// Where the line from the square's corner `k` to the diamond's reaches at `y`.
fn corner(k: usize, y: f64) -> V {
    let t = y / LENGTH;
    [0, 1, 2].map(|i| SQUARE[k][i] + t * (DIAMOND[k][i] - SQUARE[k][i]))
}

pub fn build() -> Result<Brep, String> {
    // the section at every y is the quadrilateral through the four corner lines: a face family
    // over y, one piece with its six stations at the Gauss–Lobatto nodes, each side a line whose
    // ends run along two neighbouring corner lines — its wall the ruled face between them
    let at = vec![NODES.map(|t| LENGTH / 2. * (1. + t))];
    let stations = Arc::new(Stations::new(vec![[0., LENGTH]], at)?);
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&y| {
            let (a, b) = (corner(k, y), corner((k + 1) % 4, y));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    family(&[walls], 0., None)
}
