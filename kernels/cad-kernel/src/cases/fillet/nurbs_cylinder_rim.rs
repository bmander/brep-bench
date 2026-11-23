//! The cylinder of radius 5 on the z axis from z = 0 to 10, its side one rational B-spline surface
//! (the rational quadratic circle of nine control points carried straight up, degree 1 along z,
//! its seam on the half plane y = 0, x ≥ 0) and its ends flat discs, the rim where the side meets
//! the top rounded to radius 1.
//!
//! Refused, by the kernel (0.21.0): "a fillet's run cannot be cut off where it ends: a shell of the
//! arrangement is kept on one side of a plane (`end`) and dropped on another: two of its cells
//! joined round an edge". A ball rolls on a B-spline sheet (`Surface::Offset`, the sheet's offset
//! read by its signed distance), so the round along the rim is made, but `fillet::roll` reads the
//! side's two faces (`build::grid_sheet` cuts a closed sheet into two at least) as two runs and
//! refuses cutting the run along one of them off at its ends. Tried as well, refused alike: the
//! side cut into four faces, both faces given one name, the rim picked on the other face; and the
//! side as a single face closed on its seam (`build::lofted` with the net as its one ring), whose
//! rim is one closed run, where the piece `roll` builds fails its own check ("face 1 (a): loop 0
//! turns the wrong way"): the strip of the side between the rim and the contact is laid across
//! the sheet's seam with no area in its parameters.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, face, grid_sheet};
use cad_kernel::fillet::roll;
use cad_kernel::geom::tolerance;
use cad_kernel::nurbs::Net;
use cad_kernel::sew::sewn;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const R: f64 = 5.;
const HEIGHT: f64 = 10.;
const ROUND: f64 = 1.;
const H: f64 = FRAC_1_SQRT_2;
const CIRCLE: [[f64; 2]; 9] = [
    [1., 0.], [1., 1.], [0., 1.], [-1., 1.], [-1., 0.], [-1., -1.], [0., -1.], [1., -1.], [1., 0.],
];
const CIRCLE_W: [f64; 9] = [1., H, 1., H, 1., H, 1., H, 1.];

pub fn build() -> Result<Brep, String> {
    // the net's control point (i, j) is (5cᵢ, zⱼ), weighted wᵢ
    let net = Net {
        du: 2,
        dv: 1,
        uknots: vec![0., 0., 0., 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1., 1., 1.],
        vknots: vec![0., 0., 1., 1.],
        poles: CIRCLE.iter().map(|&[x, y]| vec![[R * x, R * y, 0.], [R * x, R * y, HEIGHT]])
            .collect(),
        weights: Some(CIRCLE_W.iter().map(|&w| vec![w, w]).collect()),
    };
    // the side as one sheet closed round u, in two faces, sewn to the two discs
    let names = vec!["east".to_string(), "west".to_string()];
    let side = grid_sheet(&net, &[0., 0.5, 1.], &[0., 1.], &names, [true, false])?;
    let floor = face(&disc([0., 0., 0.], [0., 0., -1.], R))?;
    let top = face(&disc([0., 0., HEIGHT], [0., 0., 1.], R))?;
    let cylinder = sewn(&[side, floor, top], tolerance(2. * HEIGHT))?;
    // the rim picked by a point on it; the kernel rolls the ball between the side and the top
    let round = roll(&cylinder, [0., R, HEIGHT], ROUND, tolerance(cylinder.size()))?;
    let op = if round.concave { Op::Union } else { Op::Cut };
    Ok(combined(&cylinder, &round.piece, op, 0.)?)
}
