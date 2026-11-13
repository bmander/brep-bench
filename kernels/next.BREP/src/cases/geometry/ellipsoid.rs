//! The solid ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 about the origin: the kernel's ellipsoid
//! (`make_ellipsoid_brep`), its semi-axes 6, 4 and 3 along x, y and z.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_ellipsoid_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, y) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    Ok(make_ellipsoid_brep(o, x, y, [6.0, 4.0, 3.0])?)
}
