//! The ellipse x²/25 + y²/9 ≤ 1 of the plane z = 0 extruded up to z = 10: the kernel's ellipse
//! (`make_ellipse`, an exact rational curve), semi-axes 5 along x and 3 along y, as the profile of
//! its extrude.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_ellipse, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, y, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    Ok(extrude_profile_brep(&[make_ellipse(o, x, y, 5.0, 3.0)?], z, 10.0)?)
}
