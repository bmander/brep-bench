//! A ball of radius 10000 about the origin less a solid cylinder of radius 5000 about the line
//! x = 5000, y = 0, from z = −12000 to 12000, whose wall touches the sphere from inside at
//! (10000, 0, 0): `tangent/viviani-cut` a thousand times larger.
//!
//! The ball is cadrs's sphere primitive, its poles at (0, ±10000, 0) and its seam through
//! (−10000, 0, 0); the cylinder a circle extruded, its seam on the line x = 0, y = 0.
use crate::bench::{ball_oriented, cut, extrude, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let ball = ball_oriented(k, p3(0.0, 0.0, 0.0), 10000.0, v3(0.0, 1.0, 0.0), v3(-1.0, 0.0, 0.0))?;
    let on = Plane::new(p3(5000.0, 0.0, -12000.0), v3(-1.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let bore = extrude(k, &Profile::of(on, vec![Loop::circle([0.0, 0.0], 5000.0)]), 24000.0)?;
    cut(k, ball, bore)
}
