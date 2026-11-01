//! Helpers shared by the fillet, chamfer and parts cases: the kernel's Booleans by name, a point on
//! a circle (an edge is selected by a point on it), and the faces of a
//! straight edge in the order the kernel's blends read them, and the octahedron the chamfer cases
//! cut.
#![allow(unused_imports)]
use super::*;
use brep_kernel::{
    boolean_operation, make_pyramid_brep, mirror_brep, transform_brep, AffineTransform,
    AnalyticSurface, BooleanOperation, BooleanOptions, BrepSolid, FaceRecord, KernelRefusal,
};
use std::f64::consts::FRAC_1_SQRT_2;

/// `a` joined to `b`.
pub fn union(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Union, &BooleanOptions::default())
}

/// `a` less `b`.
pub fn subtract(a: &BrepSolid, b: &BrepSolid) -> Result<BrepSolid, KernelRefusal> {
    boolean_operation(a, b, BooleanOperation::Subtract, &BooleanOptions::default())
}

/// The point at `angle` (radians from +x) on the circle of `radius` about the z axis at height
/// `z`: a point that selects a circular edge, kept off the seam at angle 0.
pub fn on_circle(radius: f64, z: f64, angle: f64) -> Vec3 {
    p(radius * angle.cos(), radius * angle.sin(), z)
}

/// The octahedron |x| + |y| + |z| ≤ `size`: the kernel's regular square pyramid on the plane
/// z = 0, centred on the z axis with its apex at (0, 0, size), turned 45° about z so that its base
/// corners lie on the x and y axes, joined to its mirror image in z = 0.
pub fn octahedron(size: f64) -> Result<BrepSolid, crate::Refusal> {
    // A square of side size·√2 has its corners `size` from its centre.
    let side = size * std::f64::consts::SQRT_2;
    let pyramid = make_pyramid_brep(p(-side / 2.0, -side / 2.0, 0.0), side, size, 4)?;
    let (c, s) = (FRAC_1_SQRT_2, FRAC_1_SQRT_2);
    let turn = AffineTransform::new([
        c, -s, 0.0, 0.0, //
        s, c, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])?;
    let upper = transform_brep(&pyramid, turn, false)?;
    let lower = mirror_brep(&upper, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0))?;
    Ok(union(&upper, &lower)?)
}

/// The middle of each of the octahedron's twelve edges, for `size` 10: two coordinates ±5, the
/// third 0.
pub fn octahedron_edge_middles() -> Vec<Vec3> {
    let mut middles = Vec::new();
    for a in [5.0, -5.0] {
        for b in [5.0, -5.0] {
            middles.extend([p(a, b, 0.0), p(0.0, a, b), p(b, 0.0, a)]);
        }
    }
    middles
}

/// The faces bounding the straight edge of `solid` through `point`, in the order of the solid's
/// shells: the order in which the kernel's chamfer takes them as its face 1 and face 2.
pub fn faces_of_edge_through(solid: &BrepSolid, point: Vec3) -> Vec<&FaceRecord> {
    let at = |id: u64| solid.vertices.iter().find(|v| v.id == id).map(|v| v.point);
    let on = |a: Vec3, b: Vec3| {
        let ab = b.sub(a);
        let t = (point.sub(a).dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
        a.add(ab.scale(t)).sub(point).length() <= 1e-7
    };
    let Some(edge) = solid.edges.iter().find(|e| {
        let ends = (at(e.start_vertex_id), at(e.end_vertex_id));
        matches!(ends, (Some(a), Some(b)) if !e.degenerate && on(a, b))
    }) else {
        return Vec::new();
    };
    let mut faces: Vec<&FaceRecord> = Vec::new();
    for face in solid.shells.iter().flat_map(|shell| &shell.faces) {
        let holds = face.loops.iter().flat_map(|l| &l.coedges).any(|c| c.edge_id == edge.id);
        if holds && !faces.iter().any(|known| known.id == face.id) {
            faces.push(face);
        }
    }
    faces
}

/// Whether `face` lies in the plane z = `height`.
pub fn lies_in_plane_z(face: &FaceRecord, height: f64) -> bool {
    match face.surface.analytic() {
        Some(AnalyticSurface::Plane { origin, u_dir, v_dir, .. }) => {
            let normal = u_dir.cross(*v_dir);
            (origin.z - height).abs() <= 1e-9
                && normal.x.abs() + normal.y.abs() <= 1e-9 * normal.length()
        }
        _ => false,
    }
}
