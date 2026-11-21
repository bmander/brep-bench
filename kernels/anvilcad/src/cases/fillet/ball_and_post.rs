//! A ball of radius 10 about the origin joined to a post of radius 4 on the z axis from z = 0 to
//! 15, the circle where the post leaves the ball rounded to radius 2, adding material.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, cylinder, fillet, ring, union};

pub fn build() -> Result<Solid, String> {
    let body = union(&ball([0., 0., 0.], 10.)?, &cylinder(&Frame::world(), 4., 0., 15.)?)?;
    // the junction: the circle of radius 4 at z = √84
    fillet(&body, &[ring(4., 84f64.sqrt())], 2.)
}
