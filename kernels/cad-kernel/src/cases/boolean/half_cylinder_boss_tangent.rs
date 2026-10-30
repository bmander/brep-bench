//! A cube of side 10 at the origin united with a cylinder of radius 5 on the axis x = 10, y = 5.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    let boss = prism(&disc([SIDE, 5., 0.], [0., 0., 1.], RADIUS), 0., SIDE)?;
    Ok(combined(&cube, &boss, Op::Union, 0.)?)
}
