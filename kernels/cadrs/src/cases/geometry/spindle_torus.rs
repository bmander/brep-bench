//! A solid spindle torus centered on the origin, about the z axis: the circle through the middle
//! of its tube has radius 3 and the tube has radius 5, so the tube crosses the axis. The solid is
//! what the disk (x − 3)² + z² ≤ 25 fills as it turns, counted once: the turn of the disk's part
//! with x ≥ 0, bounded by the tube circle's outer arc, which meets the axis at (0, 0, ±4).
//!
//! cadrs's torus primitive (`Primitive::Torus`), asked for that solid where its tube is wider than
//! its ring (`Spindle::Solid`; by default it refuses a tube that crosses the axis).
use crate::bench::{p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, Primitive, Spindle};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let torus = Primitive::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: nalgebra::Unit::new_normalize(v3(0.0, 0.0, 1.0)),
        major: 3.0,
        minor: 5.0,
        spindle: Spindle::Solid,
    };
    Ok(k.primitive(&torus)?.first()?)
}
