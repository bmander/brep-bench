//! A ball of radius 10 about the origin less a ball of radius 0.8 whose centre is 10.4 from the
//! origin at height 5.501, on the half plane at 120° from +x: one dimple, its rim passing about
//! 0.00009 from the small ball's lower pole.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 10.;
const DIMPLE_RADIUS: f64 = 0.8;
const DISTANCE: f64 = 10.4;
const HEIGHT: f64 = 5.501;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    // each ball's poles on the vertical through its centre, as the case asks
    let ball = sphere([0., 0., 0.], up, RADIUS)?;
    let rho = (DISTANCE * DISTANCE - HEIGHT * HEIGHT).sqrt();
    let (s, c) = 120f64.to_radians().sin_cos();
    let cutter = sphere([rho * c, rho * s, HEIGHT], up, DIMPLE_RADIUS)?;
    Ok(combined(&ball, &cutter, Op::Cut, 0.)?)
}
