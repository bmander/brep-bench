//! A tee: a cylinder of radius 5 along the x axis from x = −15 to 15, united with one of radius 3
//! parallel to y from y = −15 to 15 whose axis is the line x = 0, z = 2, so that the two tops meet
//! tangentially at (0, 0, 5).
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const MAIN_RADIUS: f64 = 5.;
const BRANCH_RADIUS: f64 = 3.;
const BRANCH_Z: f64 = 2.;
const LENGTH: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    // each a disc at one end, square to its axis, swept along it
    let main = prism(&disc([-LENGTH / 2., 0., 0.], [1., 0., 0.], MAIN_RADIUS), 0., LENGTH)?;
    let branch =
        prism(&disc([0., -LENGTH / 2., BRANCH_Z], [0., 1., 0.], BRANCH_RADIUS), 0., LENGTH)?;
    Ok(combined(&main, &branch, Op::Union, 0.)?)
}
