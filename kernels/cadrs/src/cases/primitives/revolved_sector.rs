//! The rectangle with x from 6 to 10 and z from 0 to 4, sketched on the Front (xz) plane, revolved
//! 90° about the z axis, right-handed: from +x toward +y.
use crate::bench::{p3, revolve, v3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = Profile::of(Plane::xz(0.0), vec![Loop::rectangle(6.0, 0.0, 10.0, 4.0)]);
    revolve(k, &s, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), Some(90.0))
}
