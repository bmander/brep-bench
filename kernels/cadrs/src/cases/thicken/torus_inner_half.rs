//! A sheet: the inner half of the torus about the z axis whose tube has radius 2 about a middle
//! circle of radius 6 (the points of its surface no farther than 6 from the axis), one saddle face
//! bounded by the circles of radius 6 at z = ±2, thickened by 1 away from the middle circle,
//! toward the z axis.
//!
//! The half circle of radius 2 about (6, 0) from (6, 2) through (4, 0) to (6, −2) sketched on the
//! Front plane (its (x, z)) and surface-revolved a whole turn about z; cadrs's Thicken thickens it
//! with its arrow flipped to point toward the axis.
use crate::bench::{arc, p2, p3, v3};
use crate::shared::thicken::{open_chain, surface_revolve, thicken_toward};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Plane};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let s = open_chain(Plane::xz(0.0), vec![arc(p2(6.0, 0.0), 2.0, 90.0, 180.0)]);
    let sheet = surface_revolve(k, &s, Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)?;
    thicken_toward(k, sheet, p3(0.0, 4.0, 0.0), v3(0.0, -1.0, 0.0), 1.0)
}
