//! A cube from (−5, −5, −5) to (5, 5, 5) united with a cylinder of radius 5 on the x axis from
//! x = −10 to 10, inscribed in it.
use anvil_brep::Solid;

use crate::shared::{block, cylinder, frame, union};

pub fn build() -> Result<Solid, String> {
    // the cylinder's seam at 45° to the lines of contact, along y = z of its wall
    let axis = frame([-10., 0., 0.], [1., 0., 0.], [0., 1., 1.])?;
    union(&block([-5., -5., -5.], [5., 5., 5.])?, &cylinder(&axis, 5., 0., 20.)?)
}
