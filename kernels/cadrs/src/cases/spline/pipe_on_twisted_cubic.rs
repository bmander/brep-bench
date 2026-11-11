//! A pipe of radius 8 along the cubic Bézier curve in space with control points (0, 0, 0),
//! (12, 16, 0), (26, 8, 0) and (24, 0, 16), the section square to the curve, the ends flat: the
//! circle drawn on the plane through the origin square to the curve there, along (3, 4, 0), swept
//! along the Bézier curve (a curve in space, as cadrs's sweep takes one).
use crate::bench::{p3, v3};
use crate::shared::sweep::sweep;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let curve = PathCurve::bezier(&[p3(0.0, 0.0, 0.0), p3(12.0, 16.0, 0.0), p3(26.0, 8.0, 0.0), p3(24.0, 0.0, 16.0)]);
    let disk = Profile::of(Plane::square_to(p3(0.0, 0.0, 0.0), v3(3.0, 4.0, 0.0)), vec![Loop::circle([0.0, 0.0], 8.0)]);
    sweep(k, &disk, vec![curve])
}
