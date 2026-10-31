use crate::solids::*;
use solidcraft_geom::Seg2;

/// The block on the square −12 ≤ x, y ≤ 12 under the weighted barrel vault arching along y.
///
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

/// The vaulted block less a box whose floor is the plane z = 16, along the vault's crown, and
/// which reaches past the block on every other side.
pub fn build() -> R<Body> {
    cut(&vault()?, &block((-20.0, -20.0, 16.0), (20.0, 20.0, 40.0))?)
}
