//! A bar of square section 20 × 20 along the planar cubic Bézier curve (0, 0, 0), (16, 12, 0),
//! (18, 26, 0), (12, 34, 0) in the xy plane, two sides horizontal; flat ends.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{add, cross, scale, unit};
use std::sync::Arc;

const CONTROL: [V; 4] = [[0., 0., 0.], [16., 12., 0.], [18., 26., 0.], [12., 34., 0.]];
const HALF: f64 = 10.;
/// The curve's plane's normal: the section's upright axis at every station.
const UP: V = [0., 0., 1.];
/// How many pieces the curve's parameter is read in, six stations each: between them each corner
/// is the degree-five polynomial through its solved places, which misses the corner's true path
/// over a piece of a twenty-fourth of the parameter by under 2·10⁻¹⁰.
const PIECES: usize = 24;

/// The section at `t` on `path`: the square about the path's point there, in the plane square to
/// its tangent, its sides along the curve's normal in the xy plane and up, as its four corners.
fn square(path: &BSpline, t: f64) -> [V; 4] {
    let (centre, tangent) = path.d1(t);
    let across = unit(cross(UP, tangent));
    let at = |x: f64, z: f64| add(centre, add(scale(across, x), scale(UP, z)));
    [at(-HALF, -HALF), at(HALF, -HALF), at(HALF, HALF), at(-HALF, HALF)]
}

pub fn build() -> Result<Brep, String> {
    // the path as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let path = BSpline::new(3, knots, CONTROL.to_vec())?;
    // the section carried along the path is a face family over its parameter: the parameter in
    // pieces, each with its six stations at the Gauss–Lobatto nodes
    let step = 1. / PIECES as f64;
    let spans: Vec<[f64; 2]> =
        (0..PIECES).map(|j| [step * j as f64, step * (j + 1) as f64]).collect();
    let at = spans.iter().map(|&[a, b]| NODES.map(|t| (a + b) / 2. + t * (b - a) / 2.)).collect();
    let stations = Arc::new(Stations::new(spans, at)?);
    // each side a line whose two ends the station moves: the top and bottom walls the planes the
    // kernel finds they lie on, the upright ones swept by the path's offsets
    let walls = (0..4).map(|k| {
        let nodes = stations.at.iter().flatten().map(|&t| {
            let corners = square(&path, t);
            let (a, b) = (corners[k], corners[(k + 1) % 4]);
            vec![a[0], a[1], a[2], b[0], b[1], b[2]]
        }).collect();
        let sheet = Sheet::new(Run::Line, stations.clone(), nodes, false)?;
        Ok(Wall { sheet: Arc::new(sheet), name: "side".into() })
    }).collect::<Result<Vec<_>, String>>()?;
    family(&[walls], 0., None)
}
