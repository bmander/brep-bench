//! An L-block on the section (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) of the xz plane,
//! from y = 0 to 10, its inside edge chamfered 7 on both faces, which consumes both faces.
use anvil_brep::Solid;

use crate::shared::{chamfer, prism_xz};

pub fn build() -> Result<Solid, String> {
    let l = prism_xz(&[(0., 0.), (10., 0.), (10., 3.), (3., 3.), (3., 10.), (0., 10.)], 0., 10.)?;
    chamfer(&l, &[[3., 5., 3.]], 7.)
}
