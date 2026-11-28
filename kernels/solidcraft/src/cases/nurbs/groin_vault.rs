use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_PI_2;

/// The block of `nurbs/vault-tangent-plane`, here on solidcraft-kernel for its turn and boolean.
/// The kernel's NURBS sheets are T-splines', of odd degree only (`TSpline` refuses the vault's
/// degree 2), so the vault is built as the surface it is: degree 1 along x between two equal
/// rows, each the rational quadratic Bézier arc over (y, z) = (−12, 10), (0, 26), (12, 10) with
/// the weights 1, 3/5, 1. That is the profile's conic of apex weight 3/5 moved straight along x,
/// so the block is the section in y and z, its top that conic, extruded from x = −12 to 12.
pub fn vault() -> R<Body> {
    let plane = Plane { origin: v3(-12.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z };
    let (a, b) = (v2(12.0, 10.0), v2(-12.0, 10.0));
    let segs = vec![
        Seg2::Line { a: v2(-12.0, 0.0), b: v2(12.0, 0.0) },
        Seg2::Line { a: v2(12.0, 0.0), b: a },
        Seg2::Conic { a, apex: v2(0.0, 26.0), b, w: 0.6 },
        Seg2::Line { a: b, b: v2(-12.0, 0.0) },
    ];
    extrude(&plane, Region2 { outer: Loop2 { segs }, holes: vec![] }, 0.0, 24.0)
}

/// The vaulted block of `nurbs/vault-tangent-plane` in common with itself turned a quarter turn
/// about the z axis. (The kernel declines the common part: its exact fallback for faces that
/// touch, here the two vaults at the crown's crossing, takes planar faces only.)
pub fn build() -> R<Body> {
    let first = vault()?;
    let turned = solidcraft_kernel::transform(&first, Vec3::ZERO, Vec3::ZERO, Vec3::Z, FRAC_PI_2)?;
    common(&first, &turned)
}
