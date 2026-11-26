//! The plus-shaped plate of `offset/plus-shaped-plate-grown`, 4 thick, shrunk by 1: each arm is
//! left 1 wide, and each concave edge is rounded by the cylinder of radius 1 about it.
use super::plus_shaped_plate_grown::PLUS;
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let outline = polygon(&PLUS.map(|[x, y]| [x, y, 0.]));
    shrunk(&prism(&outline, 0., 4.)?, 1., Join::Round)
}
