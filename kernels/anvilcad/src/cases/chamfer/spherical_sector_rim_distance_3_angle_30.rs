//! A spherical sector, the ball of radius 5 about the origin inside the cone about the z axis with
//! its apex at the origin and half angle α, cos α = 3/5; its rim chamfered 3. down the cone, at
//! 30.° to it.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, chamfer_angle, cone, intersect, ring};

pub fn build() -> Result<Solid, String> {
    // the cone, from its apex up past the ball: tan α = 4. / 3.
    let sector = intersect(&ball([0., 0., 0.], 5.)?, &cone(&Frame::world(), 0., 6. * (4. / 3.), 6.)?)?;
    chamfer_angle(&sector, ring(4., 3.), ring(2., 1.5), 3., 30.)
}
