//! A block (−6, −6, −8)–(36, 26, 0) with a groove cut into its top face by a ball of radius 3
//! whose centre runs along the cubic Bézier curve (0, 0, 0), (10, 0, 0), (20, 20, 0), (30, 20, 0)
//! in the top face, from its first end to its last.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{capped, seam};
use cad_kernel::family::{family, Run, Sheet, Stations, Wall, NODES};
use cad_kernel::geom::V;
use cad_kernel::nurbs::BSpline;
use cad_kernel::recipe::FamilyWall;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::{cross, scale, unit};
use std::f64::consts::TAU;
use std::sync::Arc;

const CONTROL: [V; 4] = [[0., 0., 0.], [10., 0., 0.], [20., 20., 0.], [30., 20., 0.]];
const RADIUS: f64 = 3.;
/// The curve's plane's normal, the top face's.
const UP: V = [0., 0., 1.];
/// How many pieces the curve's parameter is read in, six stations each: between them the disc's
/// centre and its two radius vectors are the degree-five polynomials through their solved values.
const PIECES: usize = 24;

pub fn build() -> Result<Brep, String> {
    let stock = block([-6., -6., -8.], [36., 26., 0.])?;
    // the path as cad-kernel holds a Bézier segment: a cubic B-spline on clamped knots
    let knots = vec![0., 0., 0., 0., 1., 1., 1., 1.];
    let path = BSpline::new(3, knots, CONTROL.to_vec())?;
    // workaround: no sweep of a solid along a curve
    // a ball carried along a path no closed form carries sweeps the ball's disc square to the
    // path, a face family over the path's parameter, closed by the half ball beyond each end disc
    // (cad-kernel's `capped`): the parameter in pieces, each with six stations at the
    // Gauss–Lobatto nodes
    let step = 1. / PIECES as f64;
    let spans: Vec<[f64; 2]> =
        (0..PIECES).map(|j| [step * j as f64, step * (j + 1) as f64]).collect();
    let at = spans.iter().map(|&[a, b]| NODES.map(|t| (a + b) / 2. + t * (b - a) / 2.)).collect();
    let stations = Arc::new(Stations::new(spans, at)?);
    // at each station the disc's whole circle: centred on the path, square to its tangent, its
    // first radius vector turned off the top face by the kernel's `seam`, so the wall's seam and
    // the end balls' poles stay out of the plane the groove is cut from
    let nodes: Vec<Vec<f64>> = stations.at.iter().flatten().map(|&t| {
        let (centre, tangent) = path.d1(t);
        let along = unit(tangent);
        let first = scale(seam(along, unit(cross(UP, along))), RADIUS);
        let quarter = cross(along, first);
        [centre, first, quarter].concat().into_iter().chain([TAU]).collect()
    }).collect();
    let sheet = Sheet::new(Run::Round, stations.clone(), nodes.clone(), true)?;
    let tube = family(&[vec![Wall { sheet: Arc::new(sheet), name: "groove".into() }]], 0., None)?;
    let wall = FamilyWall { name: "groove".into(), run: Run::Round, closed: true, nodes };
    let swept = capped(tube, &[vec![wall]], RADIUS, "groove")?;
    Ok(combined(&stock, &swept, Op::Cut, 0.)?)
}
