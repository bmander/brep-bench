//! A cylinder of radius 6 on the z axis from z = 0 to 10, both rims rounded to radius 2, hollowed to a wall 3 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{cylinder, fillet, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let p = fillet(&cylinder(&anvil_brep::Frame::world(), 6., 0., 10.)?, &[ring(6., 0.), ring(6., 10.)], 2.)?;
    shell_open(&p, &[], 3.)
}
