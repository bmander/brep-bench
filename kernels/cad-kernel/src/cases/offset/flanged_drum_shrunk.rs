//! The cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange round its side from
//! radius 8 to 11 between z = 2.2 and z = 3.8, shrunk by 1.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let drum = prism(&disc([0., 0., 0.], [0., 0., 1.], 8.), 0., 6.)?;
    let flange = prism(&disc([0., 0., 2.2], [0., 0., 1.], 11.), 0., 1.6)?;
    shrunk(&combined(&drum, &flange, Op::Union, 0.)?, 1., Join::Round)
}
