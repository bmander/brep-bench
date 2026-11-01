//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick, shrunk
//! by 1: every point of it at least 1 inside it.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let l = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 5., 0.], [5., 5., 0.], [5., 15., 0.], [0., 15., 0.]]);
    shrunk(&prism(&l, 0., 8.)?, 1., Join::Round)
}
