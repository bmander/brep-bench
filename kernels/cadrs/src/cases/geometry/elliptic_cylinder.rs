//! A solid elliptic cylinder standing on the xy plane: the ellipse x²/25 + y²/9 ≤ 1 (semi-axes 5
//! along x and 3 along y, centered on the z axis) in the plane z = 0, extruded straight up to
//! z = 10. It has two flat elliptic ends and one curved side.
//!
//! Built as cadrs's Part Studio builds it: an ellipse sketched on the Top plane, extruded 10.
use crate::bench::extrude;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xy(0.0), vec![Loop::ellipse([0.0, 0.0], 5.0, 3.0)]);
    extrude(k, &s, 10.0)
}
