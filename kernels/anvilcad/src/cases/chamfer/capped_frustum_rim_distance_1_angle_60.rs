//! A frustum on the z axis from radius 2 at z = 0 to 4 at z = 3, capped by the part of the ball of
//! radius 5 about the origin above z = 3; the rim between them chamfered 1. down the cone, at 60.°
//! to it.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, beyond, chamfer_angle, cone, ring, subtract, union};

pub fn build() -> Result<Solid, String> {
    let cap = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 3.], [0., 0., -1.], 10.)?)?;
    let body = union(&cone(&Frame::world(), 2., 4., 3.)?, &cap)?;
    // the distance runs down the frustum's cone, through (3, 1.5)
    chamfer_angle(&body, ring(4., 3.), ring(3., 1.5), 1., 60.)
}
