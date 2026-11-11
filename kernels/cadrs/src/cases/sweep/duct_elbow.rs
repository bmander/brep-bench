//! A hollow square duct bent through a quarter turn about the z axis: its section, the square
//! 18 × 18 with a square hole 14 × 14 at its center, standing in the xz plane centered 30 out
//! along x (from 21 to 39 from the axis, z from −9 to 9), swept along the arc of radius 30 in the
//! xy plane from (30, 0, 0) to (0, 30, 0): the section drawn on the Front plane, the arc on the
//! Top plane.
use crate::bench::{arc, p2};
use crate::shared::sweep::sweep;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Profile::with_holes(
        Plane::xz(0.0),
        Loop::rectangle(21.0, -9.0, 39.0, 9.0),
        vec![Loop::rectangle(23.0, -7.0, 37.0, 7.0)],
    );
    sweep(k, &section, vec![PathCurve::sketch(Plane::xy(0.0), arc(p2(0.0, 0.0), 30.0, 0.0, 90.0))])
}
