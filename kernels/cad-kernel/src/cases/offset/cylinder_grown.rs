//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1.
use cad_kernel::build::{disc, prism};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&prism(&disc([0., 0., 0.], [0., 0., 1.], 5.), 0., 10.)?, 1., Join::Round)
}
