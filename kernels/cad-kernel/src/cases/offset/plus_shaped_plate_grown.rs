//! The plus-shaped plate through (3, 0), (6, 0), (6, 3), (9, 3), (9, 6), (6, 6), (6, 9), (3, 9),
//! (3, 6), (0, 6), (0, 3), (3, 3), 2 thick, grown by 1: its four concave edges stay sharp.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub const PLUS: [[f64; 2]; 12] = [[3., 0.], [6., 0.], [6., 3.], [9., 3.], [9., 6.], [6., 6.],
                                  [6., 9.], [3., 9.], [3., 6.], [0., 6.], [0., 3.], [3., 3.]];

pub fn build() -> Result<Brep, String> {
    let outline = polygon(&PLUS.map(|[x, y]| [x, y, 0.]));
    grown(&prism(&outline, 0., 2.)?, 1., Join::Round)
}
