//! A pipe of radius 20 along the planar cubic Bézier curve (0, 0, 0), (12, −16, 0), (32, −16, 0),
//! (44, 0, 0) in the xy plane, its section the disc square to the curve; flat ends.
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{cross, scale, unit};
use std::f64::consts::TAU;
use std::sync::Arc;

const CONTROL: [V; 4] = [[0., 0., 0.], [12., -16., 0.], [32., -16., 0.], [44., 0., 0.]];
const RADIUS: f64 = 20.;
/// The curve's plane's normal: one of the section's axes at every station.
const UP: V = [0., 0., 1.];
/// How many pieces the curve's parameter is read in, six stations each: between them the circle's
/// centre and its two radius vectors are the degree-five polynomials through their solved values,
/// which miss the true section over a piece of a twenty-fourth of the parameter by under 2·10⁻¹⁰.
const PIECES: usize = 24;

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
    // at each station the whole circle of the section: centred on the path, in the plane square
    // to its tangent there, its radius vectors along the path's normal in the xy plane and up
    let nodes = stations.at.iter().flatten().map(|&t| {
        let (centre, tangent) = path.d1(t);
        let across = scale(unit(cross(UP, tangent)), RADIUS);
        let up = scale(UP, RADIUS);
        [centre, across, up].concat().into_iter().chain([TAU]).collect()
    }).collect();
    let sheet = Sheet::new(Run::Round, stations.clone(), nodes, true)?;
    family(&[vec![Wall { sheet: Arc::new(sheet), name: "pipe".into() }]], 0., None)
}
