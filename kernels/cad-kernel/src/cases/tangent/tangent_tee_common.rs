//! The solid common to a cylinder of radius 5 along the x axis from x = −15 to 15 and one of
//! radius 3 parallel to y from y = −15 to 15 whose axis is the line x = 0, z = 2, the two tops
//! tangent at (0, 0, 5).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const FIRST_RADIUS: f64 = 5.;
const SECOND_RADIUS: f64 = 3.;
const SECOND_Z: f64 = 2.;
const LENGTH: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    // each a disc at one end, square to its axis, swept along it
    let first = prism(&disc([-LENGTH / 2., 0., 0.], [1., 0., 0.], FIRST_RADIUS), 0., LENGTH)?;
    let second =
        prism(&disc([0., -LENGTH / 2., SECOND_Z], [0., 1., 0.], SECOND_RADIUS), 0., LENGTH)?;
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
