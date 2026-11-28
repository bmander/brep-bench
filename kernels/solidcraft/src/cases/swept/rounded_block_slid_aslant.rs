//! The solid a rounded block fills as it slides along (8, 5, 3) without turning: everything within
//! 1 of the box from (1, 1, 1) to (9, 5, 3).
//!
//! solidcraft-kernel has no sweep of a solid, so the sweep is built by elementary geometry. The
//! rounded block is the box grown by the ball of radius 1, and growing commutes with sliding: what
//! it sweeps is everything within 1 of what the box sweeps. The box's sweep is, as in
//! `box-slid-aslant`, the box and its faces that lead the slide (x = 9, y = 5 and z = 3) slid the
//! whole way, each by the kernel's `sweep` along a straight path, which translates its profile;
//! the grown solid is SolidCraft's offset of the whole solid (`offset_solid`), its convex edges
//! rounded.
use crate::solids::*;
use solidcraft_kernel::{sweep, PathSeg};

pub fn build() -> R<Body> {
    let slide = v3(8.0, 5.0, 3.0);
    let mut swept = block((1.0, 1.0, 1.0), (9.0, 5.0, 3.0))?;
    // each leading face on a plane whose normal is its outward one, its sides along the box's
    // edges from the corner nearest the origin
    let faces = [
        (Plane { origin: v3(9.0, 1.0, 1.0), x: Vec3::Y, y: Vec3::Z }, rect(0.0, 0.0, 4.0, 2.0)),
        (Plane { origin: v3(1.0, 5.0, 1.0), x: Vec3::Z, y: Vec3::X }, rect(0.0, 0.0, 2.0, 8.0)),
        (Plane { origin: v3(1.0, 1.0, 3.0), x: Vec3::X, y: Vec3::Y }, rect(0.0, 0.0, 8.0, 4.0)),
    ];
    for (plane, face) in &faces {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    solidcraft_kernel::offset_solid(&swept, 1.0, true)
}
