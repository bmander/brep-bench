//! The solid over the square from (0, 0) to (10, 10), from z = 0 up to the bilinearly blended
//! Coons patch of the four straight edges from (0, 0, 5) to (10, 0, 5), (10, 10, 15), (0, 10, 5)
//! and back: the saddle z = 5 + xy/10, its walls on x = 0, x = 10, y = 0 and y = 10.
//!
//! The Coons patch of four straight edges is the bilinear patch through their corners: each of its
//! ruled blends is that patch, and so is its correction. cad-kernel 0.21.0 has no fill of a loop of
//! curves — its surfaces come from a profile swept (`build::prism`, `revolve`, `screwed`,
//! `carried`), a face family whose sections the caller solves (`family::family`), or a net the
//! caller writes (`build::sheet`, `sheets`, `grid_sheet`, `lofted`); `sew::sewn` closes faces it is
//! given and makes none — so the net is written from the four corners, degree one each way, and
//! the kernel makes its face (`sheet`) and sews it to the walls and the floor into the solid.
//!
//! What a fill would be, for curves no straight lines (`fill/coons-patch-of-four-parabolas`):
//!
//! ```text
//! nurbs::coons(edges: [&BSpline; 4]) -> Result<Net, String>
//! ```
//!
//! the bilinearly blended Coons patch of four curves closing a loop (each after the last, the
//! first and third run along u, the second and fourth along v), its net degree-raised and
//! knot-merged so its boundary rows are the edges exactly; `sheet(net)` then makes its face and
//! `sew::sewn` closes it with the walls and the floor.
use crate::sketch::polygon;
use cad_kernel::build::{face, sheet, Profile, ProfileEdge};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::nurbs::{BSpline, Net};
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
pub fn walls(edges: &[BSpline; 4]) -> Result<Vec<Brep>, String> {
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

/// The refusal where the edges are no straight lines: the fill their walls wait for.
pub fn no_fill(caps: &[Brep]) -> String {
    format!("no fill of a loop of curves: cad-kernel 0.21.0 makes no surface from four boundary \
             curves (the {} faces under them built), and the bilinearly blended Coons patch wants \
             nurbs::coons(edges: [&BSpline; 4]) -> Result<Net, String>, its face then sheet(net) \
             sewn to them", caps.len())
}

pub fn build() -> Result<Brep, String> {
    // each edge a straight Bézier segment between two corners
    let lines = |k: usize| bezier(&[CORNERS[k], CORNERS[(k + 1) % 4]]);
    let edges = [lines(0)?, lines(1)?, lines(2)?, lines(3)?];
    let mut faces = walls(&edges)?;
    // the bilinear patch through the corners: along u the first edge and the third (walked back),
    // along v the fourth and the second
    let linear = vec![0., 0., 1., 1.];
    let poles = vec![vec![CORNERS[0], CORNERS[3]], vec![CORNERS[1], CORNERS[2]]];
    let net = Net { du: 1, dv: 1, uknots: linear.clone(), vknots: linear, poles, weights: None };
    faces.push(sheet(net)?);
    Ok(sewn(&faces, tolerance(30.))?)
}
