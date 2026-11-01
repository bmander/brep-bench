//! A ball of radius 5 about the origin bounded by the standard biquadratic NURBS sphere: a 9 × 5
//! rational net whose first and last rows of poles each collapse to a pole.
use cad_kernel::build::grid_sheet;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const R: f64 = 5.;
const H: f64 = FRAC_1_SQRT_2;
const CIRCLE: [[f64; 2]; 9] = [
    [1., 0.], [1., 1.], [0., 1.], [-1., 1.], [-1., 0.], [-1., -1.], [0., -1.], [1., -1.], [1., 0.],
];
const CIRCLE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];
/// (ρ, z), the south pole to the north.
const MERIDIAN: [[f64; 2]; 5] = [[0., -R], [R, -R], [R, 0.], [R, R], [0., R]];
const MERIDIAN_W: [f64; 5] = [1., H, 1., H, 1.];

pub fn build() -> Result<Brep, String> {
    // the control point (i, j) is (ρⱼ cᵢ, zⱼ), weighted wᵢvⱼ
    let poles = CIRCLE.iter()
        .map(|c| MERIDIAN.iter().map(|m| [m[0] * c[0], m[0] * c[1], m[1]]).collect())
        .collect();
    let weights = CIRCLE_W.iter().map(|wi| MERIDIAN_W.iter().map(|wj| wi * wj).collect()).collect();
    let net = Net {
        du: 2,
        dv: 2,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 0., 0.5, 0.5, 1., 1., 1.],
        poles,
        weights: Some(weights),
    };
    // the net as one sheet closed round u (its seam the meridian at u = 0 and 1), in two bands and
    // two faces along each; the edges at v = 0 and 1 are the poles
    let names = vec!["east".to_string(), "west".to_string()];
    grid_sheet(&net, &[0., 0.5, 1.], &[0., 0.5, 1.], &names, [true, false])
}
