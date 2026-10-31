use crate::solids::*;
use solidcraft_kernel::{EdgeSpec, FaceSpec, SurfSpec, sew};

/// The solid whose section at each height is the rectangle centred on the z axis with its
/// corners on four guides: the cubic Bézier curve with control points `guide` (the corner with
/// x and y positive) and its mirror images in the xz plane, the yz plane and both.
///
/// SolidCraft's loft follows one or two guide rails, which turn and size its sections as a
/// whole, not four that set each corner. So the solid is sewn from its faces, as SolidCraft
/// sews a solid whose faces are known: each wall is the kernel's ruled surface between its two
/// guides (each a Bézier chain edge, as given), whose rulings join the guides' points of the
/// same parameter, and so of the same height; the floor and the top are flat.
pub fn on_guides(guide: [[f64; 3]; 4]) -> R<Body> {
    // the guides counter-clockwise about +z, from the corner with x and y positive
    let guides: Vec<Vec<Vec3>> = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)]
        .iter()
        .map(|&(sx, sy)| guide.iter().map(|p| v3(sx * p[0], sy * p[1], p[2])).collect())
        .collect();
    let chain = |g: &[Vec3]| EdgeSpec::Chain { degree: 3, ctrl: g.into() };
    let ring = |pts: Vec<Vec3>| -> Vec<EdgeSpec> {
        (0..pts.len()).map(|i| EdgeSpec::Line { a: pts[i], b: pts[(i + 1) % pts.len()] }).collect()
    };
    // the floor, clockwise about +z (its outward normal is −z)
    let foot = guides.iter().rev().map(|g| g[0]).collect();
    let mut faces =
        vec![FaceSpec { loops: vec![ring(foot)], surface: SurfSpec::Plane, probe: None }];
    let head: Vec<Vec3> = guides.iter().map(|g| g[3]).collect();
    // a top, unless the guides meet in a point there
    if head[0].dist(head[2]) > 1e-9 {
        faces.push(FaceSpec { loops: vec![ring(head)], surface: SurfSpec::Plane, probe: None });
    }
    for i in 0..4 {
        let (g, h) = (&guides[i], &guides[(i + 1) % 4]);
        // along the foot from g to h, up h, back along the top, down g
        let mut walk = vec![EdgeSpec::Line { a: g[0], b: h[0] }, chain(h)];
        if g[3].dist(h[3]) > 1e-9 {
            walk.push(EdgeSpec::Line { a: h[3], b: g[3] });
        }
        walk.push(chain(g).reversed());
        // the middle of the wall's foot, facing away from the z axis
        let mid = (g[0] + h[0]) * 0.5;
        let out = v3(mid.x, mid.y, 0.0).normalized().ok_or(KernelError::Invalid("wall".into()))?;
        let surface = SurfSpec::Ruled { a: chain(g), b: chain(h) };
        faces.push(FaceSpec { loops: vec![walk], surface, probe: Some((mid, out)) });
    }
    sew(&faces)
}

pub fn build() -> R<Body> {
    on_guides([[12.0, 3.2, 0.0], [18.0, -2.8, 8.0], [15.0, 0.2, 19.0], [3.0, 12.2, 24.0]])
}
