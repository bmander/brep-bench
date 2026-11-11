
//!
//! cadrs's Sweep of the disk of radius 3 on the Right plane at (−30, 0, 0) (square to the first
//! run) along the two runs drawn on the Top plane, its corner round (`SweepCorners::Round`: the
//! disk turned about the corner's axis, the z axis, from square to the first run to square to the
//! second).
use crate::bench::p2;
use crate::shared::sweep::sweep_round_corners;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let disk = Profile::of(Plane::yz(-30.0), vec![Loop::circle([0.0, 0.0], 3.0)]);
    let path = vec![
        PathCurve::sketch(Plane::xy(0.0), Curve2::line(p2(-30.0, 0.0), p2(0.0, 0.0))),
        PathCurve::sketch(Plane::xy(0.0), Curve2::line(p2(0.0, 0.0), p2(0.0, 30.0))),
    ];
    sweep_round_corners(k, &disk, path)
}
