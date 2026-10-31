//! A slab lofted through rectangles centred on the z axis, sides along x and y, from z = 0 to 24:
//! their corners on the cubic Bézier guide (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19),
//! (3, 12.2, 24) and its mirror images in the xz plane, the yz plane and both; flat ends.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const GUIDE: [V; 4] = [[12., 3.2, 0.], [18., -2.8, 8.], [15., 0.2, 19.], [3., 12.2, 24.]];
/// The signs of x and y of each guide, its corners counter-clockwise about z from the one with x
/// positive and y negative.
const MIRRORS: [[f64; 2]; 4] = [[1., -1.], [1., 1.], [-1., 1.], [-1., -1.]];

pub fn build() -> Result<Brep, String> {
    // the four guides as cad-kernel holds a Bézier segment: cubic B-splines on clamped knots, each
    // the guide's control points mirrored
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let guides = MIRRORS.iter().map(|&[sx, sy]| {
        let poles = GUIDE.iter().map(|p| [sx * p[0], sy * p[1], p[2]]).collect();
        BSpline::new(3, knots.clone(), poles)
    }).collect::<Result<Vec<_>, String>>()?;
    // The loft is a face family over the guides' common parameter t: the four guides share their
    // heights (mirror images), so at each t their points are a rectangle's corners at one height.
    // One piece with its six stations at the Gauss–Lobatto nodes: every corner is a cubic in t,
    // which the degree-five polynomial through the stations is exactly.
    let at = vec![NODES.map(|x| (1. + x) / 2.)];
    let stations = Arc::new(Stations::new(vec![[0., 1.]], at)?);
    // each side of the rectangle a straight segment between neighbouring guides
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&t| {
            let (a, b) = (guides[k].point(t), guides[(k + 1) % 4].point(t));
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    family(&[walls], 0., None)
}
