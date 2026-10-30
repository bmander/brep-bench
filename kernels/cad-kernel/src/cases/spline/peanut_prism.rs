//! A prism 10 tall on z = 0 whose section is the region inside a closed, uniform, periodic cubic
//! B-spline of twelve control points: a peanut whose waist is only 0.5 wide.
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const CONTROL: [[f64; 2]; 12] = [
    [14., 0.], [12., 8.], [4., 6.], [0., -21. / 8.], [-4., 6.], [-12., 8.],
    [-14., 0.], [-12., -8.], [-4., -6.], [0., 21. / 8.], [4., -6.], [12., -8.],
];
const DEGREE: usize = 3;
const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    // the periodic curve written the way cad-kernel holds a B-spline: its poles with the first
    // three repeated after the last, on uniform knots, so its domain [3, 15] runs once round
    let poles: Vec<_> = (0..CONTROL.len() + DEGREE)
        .map(|k| { let [x, y] = CONTROL[k % CONTROL.len()]; [x, y, 0.] })
        .collect();
    let knots = (0..poles.len() + DEGREE + 1).map(|k| k as f64).collect();
    let curve = BSpline::new(DEGREE, knots, poles)?;
    let peanut = Profile {
        origin: [0., 0., 0.],
        normal: [0., 0., 1.],
        loops: vec![vec![ProfileEdge::Spline(Arc::new(curve))]],
        names: Vec::new(),
    };
    prism(&peanut, 0., HEIGHT)
}
