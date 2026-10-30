//! A block on the square |x|, |y| ≤ 12 under a barrel vault (the rational quadratic arc through
//! (y, z) = (−12, 10), (0, 26), (12, 10), weights 1, 3/5, 1, carried along x), less everything
//! above the plane z = 16, which touches the vault along its crown.
use cad_kernel::build::{kept, prism, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::sync::Arc;

const HALF: f64 = 12.;
const ARCH: [[f64; 2]; 3] = [[-12., 10.], [0., 26.], [12., 10.]]; // (y, z)
const WEIGHTS: [f64; 3] = [1., 3. / 5., 1.];
const CROWN: f64 = 16.;

pub fn build() -> Result<Brep, String> {
    // the section in the plane x = −12 (floor, sides and the weighted arch) swept to x = 12
    let at = |y: f64, z: f64| [-HALF, y, z];
    let arch = BSpline::rational(2, vec![0., 0., 0., 1., 1., 1.],
                                 ARCH.iter().map(|&[y, z]| at(y, z)).collect(), WEIGHTS.to_vec())?;
    let [y0, z0] = ARCH[0];
    let [y1, z1] = ARCH[2];
    let section = Profile {
        origin: at(0., 0.),
        normal: [1., 0., 0.],
        loops: vec![vec![
            ProfileEdge::Line { a: at(y0, 0.), b: at(y1, 0.) },
            ProfileEdge::Line { a: at(y1, 0.), b: at(y1, z1) },
            ProfileEdge::Spline(Arc::new(arch)),
            ProfileEdge::Line { a: at(y0, z0), b: at(y0, 0.) },
        ]],
        names: Vec::new(),
    };
    let block = prism(&section, 0., 2. * HALF)?;
    // what lies on or below the plane z = 16
    kept(&block, [0., 0., CROWN], [0., 0., -1.], 0.)
}
