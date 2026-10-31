use crate::solids::*;
use solidcraft_kernel::{EdgeSpec, FaceSpec, SurfSpec, sew};

/// A flat face through `pts`, counter-clockwise seen from outside.
pub fn flat(pts: &[Vec3]) -> FaceSpec {
    let n = pts.len();
    let lp = (0..n).map(|i| EdgeSpec::Line { a: pts[i], b: pts[(i + 1) % n] }).collect();
    FaceSpec { loops: vec![lp], surface: SurfSpec::Plane, probe: None }
}

/// SolidCraft's loft pairs its sections' segments one to one (it splits the square's sides in
/// half to match the octagon's eight), so it cannot join a corner to a side. The transition's
/// faces are all flat and the description gives every corner of them, so the solid is sewn
/// from them, as SolidCraft sews a solid whose faces are known: its floor, its top, and the
/// trapezoids and triangles alternating round.
pub fn build() -> R<Body> {
    let square: Vec<Vec3> = [(-20.0, -20.0), (20.0, -20.0), (20.0, 20.0), (-20.0, 20.0)]
        .iter()
        .map(|&(x, y)| v3(x, y, 0.0))
        .collect();
    let octagon: Vec<Vec3> = [
        (-6.0, -14.0),
        (6.0, -14.0),
        (14.0, -6.0),
        (14.0, 6.0),
        (6.0, 14.0),
        (-6.0, 14.0),
        (-14.0, 6.0),
        (-14.0, -6.0),
    ]
    .iter()
    .map(|&(x, y)| v3(x, y, 30.0))
    .collect();
    // counter-clockwise about +z: side k of the square, from corner k to corner k + 1, faces
    // the octagon's side 2k; corner k + 1 faces its cut side 2k + 1
    let mut faces = vec![flat(&square.iter().rev().copied().collect::<Vec<_>>()), flat(&octagon)];
    for k in 0..4 {
        let (a, b) = (square[k], square[(k + 1) % 4]);
        let (p, q, r) = (octagon[2 * k], octagon[2 * k + 1], octagon[(2 * k + 2) % 8]);
        faces.push(flat(&[a, b, q, p]));
        faces.push(flat(&[b, r, q]));
    }
    sew(&faces)
}
