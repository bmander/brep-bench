//! The elliptic cylinder x²/100 + y²/4 ≤ 1, 0 ≤ z ≤ 10, grown by 1 to its parallel body, as
//! `offset/elliptic-cylinder-grown` grows its 5 × 3 ellipse.
use crate::solids::offset::parallel;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, make_ellipse, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (o, x, y, z) = (p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0));
    let cylinder = extrude_profile_brep(&[make_ellipse(o, x, y, 10.0, 2.0)?], z, 10.0)?;
    Ok(parallel(&cylinder, 1.0)?)
}
