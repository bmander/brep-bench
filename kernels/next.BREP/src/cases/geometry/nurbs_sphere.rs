//! A ball of radius 5 about the origin bounded by one NURBS face, the standard biquadratic sphere:
//! the 9 × 5 net (ρⱼcᵢ, zⱼ) with weights wᵢvⱼ, the unit circle cᵢ round the axis (u) and the
//! semicircle (ρⱼ, zⱼ) from the south pole to the north along the meridian (v).
//!
//! The kernel's face on that surface (`face_from_surface`) closes on its seam (its edges u = 0 and
//! u = 1, the meridian in the half plane y = 0, x ≥ 0) and at its poles (its edges v = 0 and
//! v = 1, each collapsed to a point), and is the whole boundary of the solid (`solid_from_sheets`).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{face_from_surface, solid_from_sheets, BrepSolid, NurbsSurface, Vec4};
use std::f64::consts::FRAC_1_SQRT_2 as H;

pub fn build() -> Result<BrepSolid, Refusal> {
    let circle = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0),
                  (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let circle_w = [1.0, H, 1.0, H, 1.0, H, 1.0, H, 1.0];
    let meridian = [(0.0, -5.0), (5.0, -5.0), (5.0, 0.0), (5.0, 5.0), (0.0, 5.0)];
    let meridian_w = [1.0, H, 1.0, H, 1.0];
    let net = circle
        .iter()
        .zip(circle_w)
        .map(|(&(cx, cy), w)| {
            meridian
                .iter()
                .zip(meridian_w)
                .map(|(&(rho, z), v)| Vec4::from_point(p(rho * cx, rho * cy, z), w * v))
                .collect()
        })
        .collect();
    let round = vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];
    let along = vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0];
    let sphere = NurbsSurface::new(2, 2, round, along, net)?;

    Ok(solid_from_sheets(&[face_from_surface(&sphere)?], 1e-7)?)
}
