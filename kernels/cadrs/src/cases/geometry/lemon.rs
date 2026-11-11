//! A lemon: the inner part of a spindle torus. In the xz plane, the part of the disk of radius 5
//! centered at (−3, 0, 0) with x ≥ 0, a circular segment cut off by the z axis as its chord, from
//! (0, 0, −4) to (0, 0, 4) and reaching out to (2, 0, 0), turned a whole turn about the z axis.
//!
//! On the Front (xz) plane, the segment: the arc of the circle of radius 5 about (−3, 0) from
//! (0, −4) through (2, 0) to (0, 4), closed by its chord on the z axis, revolved a whole turn
//! about z.
use crate::bench::{arc, p2, p3, revolve, v3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // The arc's ends (0, ±4) lie at angles ±arccos(3/5) about (−3, 0).
    let end = (3.0f64 / 5.0).acos().to_degrees();
    let segment = Loop::new(vec![
        arc(p2(-3.0, 0.0), 5.0, -end, 2.0 * end),
        Curve2::line(p2(0.0, 4.0), p2(0.0, -4.0)),
    ]);
    let s = Profile::of(Plane::xz(0.0), vec![segment]);
    revolve(k, &s, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)
}
