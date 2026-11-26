//! The U-shaped plate through (0, 0), (9, 0), (9, 9), (6, 9), (6, 3), (3, 3), (3, 9), (0, 9), 2
//! thick, grown by 1: the slot narrows to 1 wide, its two concave edges sharp.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const U: [[f64; 2]; 8] = [[0., 0.], [9., 0.], [9., 9.], [6., 9.], [6., 3.], [3., 3.], [3., 9.], [0., 9.]];

pub fn build() -> Result<Brep, String> {
    let outline = polygon(&U.map(|[x, y]| [x, y, 0.]));
    grown(&prism(&outline, 0., 2.)?, 1., Join::Round)
}
