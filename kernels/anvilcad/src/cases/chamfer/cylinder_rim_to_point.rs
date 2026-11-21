//! A cylinder of radius 5 on the z axis from z = 0 to 10, its top rim chamfered 5 on both faces:
//! the top shrinks to a point under a cone.
use anvil_brep::{Frame, Solid};

use crate::shared::{chamfer, cylinder, ring};

pub fn build() -> Result<Solid, String> {
    chamfer(&cylinder(&Frame::world(), 5., 0., 10.)?, &[ring(5., 10.)], 5.)
}
