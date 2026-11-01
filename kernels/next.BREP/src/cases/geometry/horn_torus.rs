//! A horn torus about the z axis: middle circle radius 5 and tube radius 5, its hole closed to a
//! point at the origin. `make_torus_brep` asks for a tube narrower than its circle, so this is the
//! disc (x − 5)² + z² ≤ 25 of the xz plane, which touches the axis at the origin, turned a whole
//! turn about the z axis.
use crate::solids::p;
use brep_kernel::{make_arc, revolve_profile_brep, BrepSolid};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, String> {
    // the rim in two halves, from the origin out to (10, 0, 0) below and back above
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let disc = [
        make_arc(p(5.0, 0.0, 0.0), x, z, 5.0, -PI, 0.0)?,
        make_arc(p(5.0, 0.0, 0.0), x, z, 5.0, 0.0, PI)?,
    ];
    revolve_profile_brep(&disc, p(0.0, 0.0, 0.0), z, 2.0 * PI)
}
