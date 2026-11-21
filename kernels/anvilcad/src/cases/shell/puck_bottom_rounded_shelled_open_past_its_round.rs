//! A cylinder of radius 6 on the z axis from z = 0 to 10, its bottom rim rounded to radius 2, shelled inward 3 thick, open at its top.
use anvil_brep::Solid;

use crate::shared::{cylinder, fillet, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let p = fillet(&cylinder(&anvil_brep::Frame::world(), 6., 0., 10.)?, &[ring(6., 0.)], 2.)?;
    shell_open(&p, &[ring(1., 10.)], 3.)
}
