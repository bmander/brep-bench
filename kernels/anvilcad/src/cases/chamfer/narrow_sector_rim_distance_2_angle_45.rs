//! A spherical sector, the ball of radius 5 about the origin inside the cone about the z axis with
//! its apex at the origin and half angle α, cos α = 4/5; its rim chamfered 2. down the cone, at
//! 45.° to it.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, chamfer_angle, cone, intersect, ring};

pub fn build() -> Result<Solid, String> {
    // the cone, from its apex up past the ball: tan α = 3. / 4.
    let sector = intersect(&ball([0., 0., 0.], 5.)?, &cone(&Frame::world(), 0., 6. * (3. / 4.), 6.)?)?;
    chamfer_angle(&sector, ring(3., 4.), ring(1.5, 2.), 2., 45.)
}
