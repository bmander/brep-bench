//! A ball of radius 5 about the origin joined to a seat, the cylinder of radius 8 on the z axis
//! from z = −6 to 5 hollowed above the cone z = 1 + ρ/2; the concave circle where the seat's cone
//! meets the ball (radius 4, z = 3) chamfered 2. up the cone, at 30.° to it.
use anvil_brep::{Frame, Solid};

use crate::shared::{at, ball, chamfer_angle, cone, cylinder, ring, subtract, union};

pub fn build() -> Result<Solid, String> {
    // the hollow: the cone from its apex at (0, 0, 1), widening by 2 a unit up, past the plate
    let hollow = cone(&at([0., 0., 1.]), 0., 12., 6.)?;
    let seat = subtract(&cylinder(&Frame::world(), 8., -6., 5.)?, &hollow)?;
    let body = union(&seat, &ball([0., 0., 0.], 5.)?)?;
    // the distance runs up the seat's cone, through (6, 4)
    chamfer_angle(&body, ring(4., 3.), ring(6., 4.), 2., 30.)
}
