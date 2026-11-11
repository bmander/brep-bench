//! A solid duct along the y axis from the square A (10, 0, −10), B (−10, 0, −10), C (−10, 0, 10),
//! D (10, 0, 10) at y = 0 to the diamond A′ (0, 40, −12), B′ (−12, 40, 0), C′ (0, 40, 12),
//! D′ (12, 40, 0) at y = 40, each corner running straight to its namesake: cadrs's Loft (Solid)
//! between the two quadrilaterals, each drawn on a plane square to the y axis (sketch (x, z)),
//! corner for corner from A (two sections give ruled faces).
use crate::shared::loft::loft;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let square =
        Profile::of(Plane::xz(0.0), vec![Loop::polygon(&[[10.0, -10.0], [-10.0, -10.0], [-10.0, 10.0], [10.0, 10.0]])]);
    let diamond =
        Profile::of(Plane::xz(40.0), vec![Loop::polygon(&[[0.0, -12.0], [-12.0, 0.0], [0.0, 12.0], [12.0, 0.0]])]);
    loft(k, vec![square, diamond])
}
