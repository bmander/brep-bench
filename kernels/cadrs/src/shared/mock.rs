//! Mock-up only (the hidden-slice experiment): one rigid placement aslant to every axis, applied
//! to whole cases (their bodies, and the points, vectors and axes their motions are given by), so
//! that a placed case must measure as the case does.
use crate::bench::{moved, turned, unit, v3, P3, V3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};
use nalgebra::Rotation3;

const AXIS: [f64; 3] = [0.3, -0.7, 0.65];
const DEGREES: f64 = 37.3;
const SHIFT: [f64; 3] = [3.1, -2.4, 1.7];

fn rotation() -> Rotation3<f64> {
    Rotation3::from_axis_angle(&unit(v3(AXIS[0], AXIS[1], AXIS[2])), DEGREES.to_radians())
}

/// A body placed: turned about the line through the origin along AXIS, then moved by SHIFT.
pub fn place(k: &mut DefaultKernel, b: BodyId) -> Result<BodyId, Refusal> {
    let a = Axis::new(P3::origin(), v3(AXIS[0], AXIS[1], AXIS[2]));
    let b = turned(k, b, a, DEGREES)?;
    moved(k, b, v3(SHIFT[0], SHIFT[1], SHIFT[2]))
}

pub fn point(p: P3) -> P3 {
    rotation() * p + v3(SHIFT[0], SHIFT[1], SHIFT[2])
}

pub fn vector(v: V3) -> V3 {
    rotation() * v
}

pub fn axis(a: Axis) -> Axis {
    Axis::new(point(a.origin), vector(a.dir.into_inner()))
}
