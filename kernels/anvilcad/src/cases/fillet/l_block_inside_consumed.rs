//! An L-block on the section (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) of the xz plane,
//! from y = 0 to 10, its inside edge rounded to radius 7, which consumes both faces at it.
use anvil_brep::Solid;

use crate::shared::{fillet, prism_xz};

pub fn build() -> Result<Solid, String> {
    let l = prism_xz(&[(0., 0.), (10., 0.), (10., 3.), (3., 3.), (3., 10.), (0., 10.)], 0., 10.)?;
    fillet(&l, &[[3., 5., 3.]], 7.)
}
