//! A ball of radius 5 about the origin bounded by one NURBS face, the standard biquadratic sphere:
//! the 9 × 5 net (ρⱼcᵢ, zⱼ) with weights wᵢvⱼ, the unit circle cᵢ round the axis (u) and the
//! semicircle (ρⱼ, zⱼ) from the south pole to the north along the meridian (v).
//!
//! The face is closed on its seam (its edges u = 0 and u = 1, the meridian in the half plane
//! y = 0, x ≥ 0) and at its poles (its edges v = 0 and v = 1, each collapsed to a point, a
//! degenerate edge), and is the whole boundary of the solid.
use crate::solids::geometry::{uv_line, Records};
use crate::solids::p;
use brep_kernel::{BrepSolid, NurbsSurface, Vec4};
use std::f64::consts::FRAC_1_SQRT_2 as H;

pub fn build() -> Result<BrepSolid, String> {
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

    let mut b = Records::new();
    let south = b.vertex(sphere.evaluate(0.0, 0.0)?);
    let north = b.vertex(sphere.evaluate(0.0, 1.0)?);
    let seam = b.edge(sphere.iso_curve_u(0.0)?, south, north)?;
    let (south_pole, north_pole) = (b.pole(south)?, b.pole(north)?);
    b.face(sphere, vec![
        (south_pole, true, uv_line(0.0, 0.0, 1.0, 0.0)?),
        (seam, true, uv_line(1.0, 0.0, 1.0, 1.0)?),
        (north_pole, true, uv_line(1.0, 1.0, 0.0, 1.0)?),
        (seam, false, uv_line(0.0, 1.0, 0.0, 0.0)?),
    ]);
    b.solid(0)
}
