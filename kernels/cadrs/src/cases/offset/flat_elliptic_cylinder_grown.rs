//! The elliptic cylinder x²/100 + y²/4 ≤ 1 from z = 0 to z = 10, grown by 1: every point within 1
//! of it. Its side moves out along its normals (the curve moved outward doesn't fold, though the
//! ellipse curves tighter than 1 at the ends of its long axis), its ends to z = −1 and z = 11,
//! and each rim becomes the outer quarter of a tube of radius 1 about it.
//!
//! The ellipse extruded; then cadrs's offset with rounded edges.
use crate::bench::extrude;
use crate::shared::offset::grown;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xy(0.0), vec![Loop::ellipse([0.0, 0.0], 10.0, 2.0)]);
    let c = extrude(k, &s, 10.0)?;
    grown(k, c, 1.0)
}
