//! A solid torus about z, its tube of radius 2 about a middle circle of radius 4, grown by 3: past
//! its hole, an apple.
use cad_kernel::build::torus;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&torus([0., 0., 0.], [0., 0., 1.], 4., 2.)?, 3., Join::Round)
}
