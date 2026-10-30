//! The solid common to two cylinders of radius 5 and length 30 crossing at right angles, one
//! along the x axis and one parallel to y on the line x = 0, z = 9.99: they overlap 0.01 deep.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const LENGTH: f64 = 30.;
const OFFSET: f64 = 9.99;

pub fn build() -> Result<Brep, String> {
    // each a disc at one end, square to its axis, swept along it
    let first = prism(&disc([-LENGTH / 2., 0., 0.], [1., 0., 0.], RADIUS), 0., LENGTH)?;
    let second = prism(&disc([0., -LENGTH / 2., OFFSET], [0., 1., 0.], RADIUS), 0., LENGTH)?;
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
