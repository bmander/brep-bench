//! A square-to-octagon transition: the square ±20 at z = 0 to the octagon at z = 30 (the square
//! ±14 with its corners cut off), each side of the square joined by a flat trapezoid to the
//! octagon's side parallel to it, each corner by a flat triangle to the cut side facing it.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const HEIGHT: f64 = 30.;
/// The square's corners, counter-clockwise about z from (20, −20).
const SQUARE: [[f64; 2]; 4] = [[20., -20.], [20., 20.], [-20., 20.], [-20., -20.]];
/// The octagon's corners, counter-clockwise from (6, −14): the cut side from corner `2k` to
/// `2k + 1` faces the square's corner `k`, and the side from `2k + 1` to `2k + 2` is parallel to
/// the square's side from its corner `k` to `k + 1`.
const OCTAGON: [[f64; 2]; 8] = [
    [6., -14.], [14., -6.], [14., 6.], [6., 14.], [-6., 14.], [-14., 6.], [-14., -6.], [-6., -14.],
];

/// The point a fraction `t` of the way up the straight line from `a` in the floor to `b` in the
/// top.
fn up(a: [f64; 2], b: [f64; 2], t: f64) -> V {
    [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), HEIGHT * t]
}

pub fn build() -> Result<Brep, String> {
    // A loft from one section to the other is a face family, one piece with its six stations at
    // the Gauss–Lobatto nodes. The section has eight sides, each a line whose ends run straight
    // from a corner of the square to a corner of the octagon; the square's corner k is both ends
    // of the side that grows into the octagon's cut side facing it, an edge vanished at the foot.
    // Every wall lies in one plane, which the kernel finds from the side at the family's start,
    // so the family runs down from the octagon, where every side has its length: over the
    // fraction s of the way down, at t = 1 − s of the way up.
    let at = vec![NODES.map(|x| (1. + x) / 2.)];
    let stations = Arc::new(Stations::new(vec![[0., 1.]], at)?);
    // each side's ends: where it starts on the square, where it ends on the octagon
    let sides = (0..8).map(|i| {
        let k = i / 2;
        let foot = if i % 2 == 0 { [SQUARE[k], SQUARE[k]] } else { [SQUARE[k], SQUARE[(k + 1) % 4]] };
        (foot, [OCTAGON[i], OCTAGON[(i + 1) % 8]])
    });
    let walls = sides.map(|(foot, top)| {
        let nodes = stations.at.iter().flatten().map(|&s| {
            let t = 1. - s;
            let (a, b) = (up(foot[0], top[0], t), up(foot[1], top[1], t));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    // an edge no longer than a billionth at an end has vanished there
    family(&[walls], 1e-9, None)
}
