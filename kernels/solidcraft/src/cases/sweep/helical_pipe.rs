use crate::solids::*;
use solidcraft_kernel::sweep_helix;
use std::f64::consts::TAU;

/// The helix's radius, rise per turn and turns, and the pipe's radius.
pub const RADIUS: f64 = 10.0;
pub const PITCH: f64 = 5.0;
pub const TURNS: f64 = 3.0;
pub const PIPE: f64 = 1.0;

/// The pipe: SolidCraft's coil, its helical sweep, of a circle drawn square to the helix where
/// it starts at (10, 0, 0). The sweep is a screw motion about the z axis, which carries the
/// helix's tangent along with the section, so the section stays square to the helix all the way
/// and the ends are flat.
pub fn pipe() -> R<Body> {
    let tangent = v3(0.0, TAU * RADIUS, PITCH);
    let plane = Plane::from_normal(v3(RADIUS, 0.0, 0.0), tangent)
        .ok_or_else(|| KernelError::Invalid("the section's plane".into()))?;
    sweep_helix(&plane, &disc((0.0, 0.0), PIPE), Vec3::ZERO, Vec3::Z, PITCH, TURNS)
}

pub fn build() -> R<Body> {
    pipe()
}
