//! A square of side 2 swept along 350° of the arc of radius 10 about the z axis in the xy plane,
//! anticlockwise from (10, 0, 0), starting as the square from (9, 0, −1) to (11, 0, 1), square to
//! the path, the ends flat: the part of the flat ring between radius 9 and 11, from z = −1 to 1,
//! from 0° to 350° about the z axis.
//!
//! The square drawn on the plane through (10, 0, 0) square to the arc there (its normal +y),
//! swept along the arc drawn on the Top plane.
use crate::bench::{arc, p2, p3, v3};
use crate::shared::sweep::sweep;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, PathCurve, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let across = Plane::new(p3(10.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
    let section = Profile::of(across, vec![Loop::rectangle(-1.0, -1.0, 1.0, 1.0)]);
    sweep(k, &section, vec![PathCurve::sketch(Plane::xy(0.0), arc(p2(0.0, 0.0), 10.0, 0.0, 350.0))])
}
