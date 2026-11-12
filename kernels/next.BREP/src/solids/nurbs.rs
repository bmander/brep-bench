//! Helpers shared by the NURBS cases that need a solid whose side is one rational B-spline face: a
//! cylinder written as the tensor product of the rational quadratic circle and a line.
#![allow(unused_imports)]
use super::*;
use crate::Refusal;
use brep_kernel::{
    face_from_surface, face_from_surface_trimmed, make_plane, solid_from_sheets, BrepSolid,
    NurbsCurve, NurbsSurface, Vec4,
};
use std::f64::consts::FRAC_1_SQRT_2 as H;

/// The standard rational quadratic unit circle's nine control points, counter-clockwise from
/// (1, 0) and back: its weights are 1, √2/2, … on the knots (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1).
pub const CIRCLE_FROM_X: [(f64, f64); 9] = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0),
    (-1.0, 0.0), (-1.0, -1.0), (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];

/// The solid cylinder of `radius` about the z axis from z = `z0` to z = `z1`, its side one NURBS
/// face: around the axis (u) the rational quadratic circle of the nine control points `circle`
/// (counter-clockwise, its first and last the same, weights 1, √2/2, … alternating) scaled by
/// `radius`, along it (v) degree 1 from z0 to z1. The side is closed on its seam, the line through
/// its first control point, and the ends are flat discs, each bounded by its rim, the side's edge
/// v = 0 or v = 1. No cylinder is used.
///
/// The side is the kernel's face on that surface (`face_from_surface`, which finds its seam), each
/// end the kernel's flat face inside its rim (`face_from_surface_trimmed`), and the three are
/// joined into the solid they bound (`solid_from_sheets`).
pub fn nurbs_cylinder(radius: f64, z0: f64, z1: f64, circle: &[(f64, f64); 9])
    -> Result<BrepSolid, Refusal> {
    let weights = [1.0, H, 1.0, H, 1.0, H, 1.0, H, 1.0];
    let net = circle
        .iter()
        .zip(weights)
        .map(|(&(cx, cy), w)| {
            [z0, z1].iter().map(|&z| Vec4::from_point(p(radius * cx, radius * cy, z), w)).collect()
        })
        .collect();
    let round = vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];
    let side = NurbsSurface::new(2, 1, round, vec![0.0, 0.0, 1.0, 1.0], net)?;
    let end = |z: f64, v: f64| -> Result<BrepSolid, Refusal> { disc(radius, z, side.iso_curve_v(v)?) };
    Ok(solid_from_sheets(&[face_from_surface(&side)?, end(z0, 0.0)?, end(z1, 1.0)?], 1e-7)?)
}

/// The flat face inside `rim`, a closed curve in the plane z = `z` within `radius` of the z axis.
pub fn disc(radius: f64, z: f64, rim: NurbsCurve) -> Result<BrepSolid, Refusal> {
    let (ex, ey) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let plane = make_plane(p(-radius, -radius, z), ex, ey, 2.0 * radius, 2.0 * radius)?;
    Ok(face_from_surface_trimmed(&plane, &[vec![rim]])?)
}
