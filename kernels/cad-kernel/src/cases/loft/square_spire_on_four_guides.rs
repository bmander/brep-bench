//! A square spire lofted through squares centred on the z axis, sides along x and y, from the
//! square 18 × 18 at z = 0 to the apex (0, 0, 24): their corners on the cubic Bézier ridge
//! (9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24) and its mirror images in the xz plane, the yz
//! plane and both; a flat floor.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const RIDGE: [V; 4] = [[9., 9., 0.], [15., 15., 8.], [12., 12., 19.], [0., 0., 24.]];
/// The signs of x and y of each ridge, its corners counter-clockwise about z from the one with x
/// positive and y negative.
const MIRRORS: [[f64; 2]; 4] = [[1., -1.], [1., 1.], [-1., 1.], [-1., -1.]];

pub fn build() -> Result<Brep, String> {
    // the four ridges as cad-kernel holds a Bézier segment: cubic B-splines on clamped knots, each
    // the ridge's control points mirrored
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let ridges = MIRRORS.iter().map(|&[sx, sy]| {
        let poles = RIDGE.iter().map(|p| [sx * p[0], sy * p[1], p[2]]).collect();
        BSpline::new(3, knots.clone(), poles)
    }).collect::<Result<Vec<_>, String>>()?;
    // The loft is a face family over the ridges' common parameter t: the four ridges share their
    // heights (mirror images), so at each t their points are a square's corners at one height.
    // One piece with its six stations at the Gauss–Lobatto nodes: every corner is a cubic in t,
    // which the degree-five polynomial through the stations is exactly.
    let at = vec![NODES.map(|x| (1. + x) / 2.)];
    let stations = Arc::new(Stations::new(vec![[0., 1.]], at)?);
    // each side of the square a straight segment between neighbouring ridges
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&t| {
            let (a, b) = (ridges[k].point(t), ridges[(k + 1) % 4].point(t));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    // at the top every side has vanished (no longer than a billionth): the section closes to the
    // apex, with no cap
    family(&[walls], 1e-9, None)
}
