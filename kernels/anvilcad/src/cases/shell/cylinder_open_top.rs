//! A cylinder of radius 5 on the z axis from z = 0 to 10, shelled inward 1 thick, open at its top.
use anvil_brep::Solid;

use crate::shared::{cylinder, ring, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let c = cylinder(&anvil_brep::Frame::world(), 5., 0., 10.)?;
    shell_open(&c, &[ring(2., 10.)], 1.)
}
