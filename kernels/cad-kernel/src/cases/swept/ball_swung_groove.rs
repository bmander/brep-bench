//! A block (−6, −6, −8)–(18, 18, 0) with a groove cut into its top face by a ball of radius 3 whose
//! centre swings a quarter turn about the z axis, right-handed, from (12, 0, 0) to (0, 12, 0).
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::canal;
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_PI_2;

const RADIUS: f64 = 3.;
const SWING_RADIUS: f64 = 12.;
const SWING: f64 = FRAC_PI_2;

pub fn build() -> Result<Brep, String> {
    let stock = block([-6., -6., -8.], [18., 18., 0.])?;
    // everything the ball passes through as it turns about z, by cad-kernel's own sweep of a
    // ball: the canal of a turn with no advance, the disc square to the path turned (a quarter of
    // a torus) with the half ball beyond each end disc
    let start = [SWING_RADIUS, 0., 0.];
    let swept = canal(start, RADIUS, [0., 0., 0.], [0., 0., 1.], SWING, 0., "groove")?;
    Ok(combined(&stock, &swept, Op::Cut, 0.)?)
}
