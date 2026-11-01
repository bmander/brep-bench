//! The rectangle 6 ≤ x ≤ 10, 0 ≤ z ≤ 4 of the xz plane, revolved a full turn about the z axis.
use crate::solids::{p, polygon};
use brep_kernel::{revolve_profile_brep, BrepSolid, Vec3};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, String> {
    let rectangle = polygon(&[p(6.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 4.0), p(6.0, 0.0, 4.0)])?;
    revolve_profile_brep(&rectangle, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), TAU)
}
