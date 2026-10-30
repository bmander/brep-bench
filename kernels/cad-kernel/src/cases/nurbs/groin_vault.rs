//! The common part of two barrel-vaulted blocks on the square |x|, |y| ≤ 12, the second the
//! first turned a quarter turn about z. Each vault's top is the rational quadratic arc through
//! (y, z) = (−12, 10), (0, 26), (12, 10) with weights 1, 3/5, 1, carried straight along x.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::geom::Rigid;
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;
use std::sync::Arc;

const HALF: f64 = 12.;
const ARCH: [[f64; 2]; 3] = [[-12., 10.], [0., 26.], [12., 10.]]; // (y, z)
const WEIGHTS: [f64; 3] = [1., 3. / 5., 1.];

/// The first block: its section in the plane x = −12 (floor, sides and the weighted arch)
/// swept along +x to x = 12, so its top is the arch carried along x, degree 1 that way.
fn vault() -> Result<Brep, String> {
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
    prism(&section, 0., 2. * HALF)
}

pub fn build() -> Result<Brep, String> {
    let first = vault()?;
    let second = first.moved(&Rigid::turn([0., 0., 0.], [0., 0., 1.], FRAC_PI_2));
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
