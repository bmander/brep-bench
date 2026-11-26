//! A solid torus about z, its tube of radius 2 about a middle circle of radius 4, grown by 2: the
//! horn torus, its tube of radius 4 just closing the hole at the origin.
use cad_kernel::build::torus;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&torus([0., 0., 0.], [0., 0., 1.], 4., 2.)?, 2., Join::Round)
}
