//! A prism on a closed cubic B-spline, the rim of its top chamfered 1 on both faces.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{chamfer, e, periodic_bspline, region, rim};

pub fn build() -> Result<Solid, String> {
    let bean = periodic_bspline(&[(-10., -5.), (0., -7.), (10., -5.), (12., 4.), (4., 7.), (0., 3.), (-4., 7.), (-12., 4.)], 3);
    let boss = extrude::extrude(&Frame::world(), &region(&[bean.clone()])?, 0., 6.).map_err(e)?;
    chamfer(&boss, &rim(&bean, 6.), 1.)
}
