//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1: the ellipse x²/36 + z²/9 ≤ 1 of the xz plane turned
//! about the z axis. next.BREP has no ellipse among its curves, so the half disc of radius 6 is
//! turned a whole turn and the ball it makes is squashed by ½ along z, an affine map that takes
//! the circle to the ellipse and the kernel applies exactly to the rational poles.
use crate::solids::geometry::stretch;
use brep_kernel::{make_arc, make_line, revolve_profile_brep, transform_brep, BrepSolid, Vec3};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, String> {
    let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let half_disc = [
        make_arc(origin, x, z, 6.0, -FRAC_PI_2, FRAC_PI_2)?,
        make_line(Vec3::new(0.0, 0.0, 6.0), Vec3::new(0.0, 0.0, -6.0))?,
    ];
    let ball = revolve_profile_brep(&half_disc, origin, z, TAU)?;
    transform_brep(&ball, stretch(1.0, 1.0, 0.5)?, false)
}
