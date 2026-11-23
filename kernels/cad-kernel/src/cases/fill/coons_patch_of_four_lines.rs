//! The solid over the square from (0, 0) to (10, 10), from z = 0 up to the bilinearly blended
//! Coons patch of the four straight edges from (0, 0, 5) to (10, 0, 5), (10, 10, 15), (0, 10, 5)
//! and back: the saddle z = 5 + xy/10, its walls on x = 0, x = 10, y = 0 and y = 10.
//!
//! The Coons patch of four straight edges is the bilinear patch through their corners: each of its
//! ruled blends is that patch, and so is its correction. cad-kernel fills the loop
//! (`nurbs::coons`, the bilinearly blended Coons patch of four curves end to end, the first and
//! third along u, the second and fourth along v), its net degree one each way with the edges as
//! its boundary rows; the kernel makes its face (`sheet`) and sews it to the walls and the floor
//! into the solid (`sew::sewn`).
use crate::sketch::polygon;
use cad_kernel::build::{face, sheet, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::{coons, BSpline};
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::distance;
use std::sync::Arc;

/// The loop's corners, end to end round the square.
const CORNERS: [V; 4] = [[0., 0., 5.], [10., 0., 5.], [10., 10., 15.], [0., 10., 5.]];

/// The Bézier curve on `poles`, as cad-kernel holds one: a B-spline on clamped knots.
pub fn bezier(poles: &[V]) -> Result<BSpline, String> {
    let d = poles.len() - 1;
    BSpline::new(d, [vec![0.; d + 1], vec![1.; d + 1]].concat(), poles.to_vec())
}

/// What the fill of `edges` (four curves end to end round the square from (0, 0) to (10, 10),
/// each in the upright plane over its side) caps: the wall under each edge down to z = 0, and the
/// floor. Refused where the edges do not close.
fn walls(edges: &[BSpline; 4]) -> Result<Vec<Brep>, String> {
    let ends = |c: &BSpline| { let [a, b] = c.domain(); (c.point(a), c.point(b)) };
    for k in 0..4 {
        let (_, end) = ends(&edges[k]);
        let (start, _) = ends(&edges[(k + 1) % 4]);
        if distance(end, start) > 1e-12 {
            return Err(format!("edge {k} ends where the next does not start"));
        }
    }
    let down = |p: V| [p[0], p[1], 0.];
    let mut faces = edges.iter().map(|c| {
        let (a, b) = ends(c);
        // the wall's outward normal: square to its side, away from the square's middle
        let mid = [0.5 * (a[0] + b[0]) - 5., 0.5 * (a[1] + b[1]) - 5., 0.];
        let out = mid.map(|x| x / 5.);
        face(&Profile { origin: down(a), normal: out, names: Vec::new(), loops: vec![vec![
            ProfileEdge::Spline(Arc::new(c.clone())),
            ProfileEdge::Line { a: b, b: down(b) },
            ProfileEdge::Line { a: down(b), b: down(a) },
            ProfileEdge::Line { a: down(a), b: a },
        ]] })
    }).collect::<Result<Vec<_>, String>>()?;
    let mut floor = polygon(&[[0., 0., 0.], [0., 10., 0.], [10., 10., 0.], [10., 0., 0.]]);
    floor.normal = [0., 0., -1.];
    faces.push(face(&floor)?);
    Ok(faces)
}

/// The solid the Coons patch of `edges` caps: the patch's sheet sewn to the walls under the
/// edges and the floor.
pub fn capped(edges: &[BSpline; 4]) -> Result<Brep, String> {
    let mut faces = walls(edges)?;
    faces.push(sheet(coons(edges.each_ref())?)?);
    Ok(sewn(&faces, tolerance(30.))?)
}

pub fn build() -> Result<Brep, String> {
    // each edge a straight Bézier segment between two corners
    let lines = |k: usize| bezier(&[CORNERS[k], CORNERS[(k + 1) % 4]]);
    let edges = [lines(0)?, lines(1)?, lines(2)?, lines(3)?];
    capped(&edges)
}
