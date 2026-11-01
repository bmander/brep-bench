//! The solid ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 about the origin. next.BREP has no ellipsoid, so
//! it is the ball of radius 1 stretched by 6, 4 and 3 along x, y and z: an affine map, which the
//! kernel applies exactly to the ball's rational poles.
use crate::solids::geometry::stretch;
use brep_kernel::{make_sphere_brep, transform_brep, BrepSolid, Vec3};

pub fn build() -> Result<BrepSolid, String> {
    let ball = make_sphere_brep(Vec3::new(0.0, 0.0, 0.0), 1.0, Vec3::new(0.0, 0.0, 1.0))?;
    transform_brep(&ball, stretch(6.0, 4.0, 3.0)?, false)
}
