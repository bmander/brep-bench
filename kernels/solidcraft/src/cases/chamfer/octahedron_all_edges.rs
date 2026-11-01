//! The kernel's chamfer of several edges cuts each with a prism whose only face inside the body
//! is the chamfer plane, and cuts their union, so neighbouring chamfers meet in a mitre: it has
//! no corner that closes where four chamfers meet with a flat face, and this answers the mitred
//! solid of `chamfer/octahedron-all-edges-mitred`, 20 faces, without the six corner squares.
use crate::solids::*;
use solidcraft_kernel::HalfSpace;

/// The octahedron |x| + |y| + |z| <= 10: the kernel's convex polyhedron of its eight faces'
/// half-spaces, (±x ± y ± z)/√3 <= 10/√3.
pub fn octahedron() -> R<Body> {
    let mut faces = Vec::new();
    for sx in [1.0, -1.0] {
        for sy in [1.0, -1.0] {
            for sz in [1.0, -1.0] {
                let n = v3(sx, sy, sz) * (1.0 / 3f64.sqrt());
                faces.push(HalfSpace { n, d: 10.0 / 3f64.sqrt() });
            }
        }
    }
    solidcraft_kernel::convex_polyhedron(&faces)
}

/// The middle of each of the octahedron's twelve edges, the one between the corners on two axes
/// at signs `a` and `b`, such as (5, 5, 0) between (10, 0, 0) and (0, 10, 0).
pub fn middles() -> Vec<Vec3> {
    let mut edges = Vec::new();
    for a in [5.0, -5.0] {
        for b in [5.0, -5.0] {
            edges.extend([v3(a, b, 0.0), v3(0.0, a, b), v3(b, 0.0, a)]);
        }
    }
    edges
}

pub fn build() -> R<Body> {
    // All twelve edges in one chamfer, √6 across each face.
    solidcraft_kernel::chamfer(&octahedron()?, &middles(), 6f64.sqrt())
}
