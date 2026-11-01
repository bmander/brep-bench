//! The solid between the discs of radius 5 about the z axis at z = −5 and z = 5 and the surface
//! ruled between their rims, the point at θ below joined to the point at θ + 90° above: a loft of
//! the two circles, each point of one joined by a straight line to the point at the same
//! parameter of the other.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{loft_profile_brep, make_arc, BrepSolid, NurbsCurve, Vec3};
use std::f64::consts::PI;

/// The circle of radius 5 about the z axis at height `z`, from the angle of `start` round, as its
/// two halves (a loft's section has at least two curves).
fn rim(z: f64, start: Vec3) -> Result<Vec<NurbsCurve>, String> {
    let across = p(0.0, 0.0, 1.0).cross(start);
    let centre = p(0.0, 0.0, z);
    Ok(vec![
        make_arc(centre, start, across, 5.0, 0.0, PI)?,
        make_arc(centre, start, across, 5.0, PI, 2.0 * PI)?,
    ])
}

pub fn build() -> Result<BrepSolid, Refusal> {
    // the upper rim parametrized as the lower one, a quarter turn on
    Ok(loft_profile_brep(&[rim(-5.0, p(1.0, 0.0, 0.0))?, rim(5.0, p(0.0, 1.0, 0.0))?])?)
}
