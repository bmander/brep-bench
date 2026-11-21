//! A plate, the cylinder of radius 8 on the z axis from z = −6 to 0, joined to a ball of radius 5
//! about the origin; the concave circle where the ball leaves the plate's top chamfered 1. out on
//! the plate, at 50.° to it.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, chamfer_angle, cylinder, ring, union};

pub fn build() -> Result<Solid, String> {
    let body = union(&cylinder(&Frame::world(), 8., -6., 0.)?, &ball([0., 0., 0.], 5.)?)?;
    chamfer_angle(&body, ring(5., 0.), ring(7.5, 0.), 1., 50.)
}
