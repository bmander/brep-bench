//! The solid the box from the origin to (10, 6, 4) fills as it slides along (8, 5, 3) without
//! turning.
//!
//! solidcraft-kernel has no sweep of a solid, so the slide is built from the box and its faces by
//! elementary geometry. A point the box passes over that is outside the box where it starts is
//! reached first by one of the faces the box leads with, those whose outward normals point along
//! the slide: x = 10, y = 6 and z = 4. So the solid is the box where it starts and those three
//! faces each slid the whole way, a slanted prism apiece; the box where it ends is their far end.
//! The kernel's `sweep` along a straight path translates its profile, the face, along it.
use crate::solids::*;
use solidcraft_kernel::{PathSeg, sweep};

pub fn build() -> R<Body> {
    let slide = v3(8.0, 5.0, 3.0);
    let mut swept = block((0.0, 0.0, 0.0), (10.0, 6.0, 4.0))?;
    // each leading face on a plane whose normal is its outward one, its sides along the box's
    // edges from the corner nearest the origin
    let faces = [
        (Plane { origin: v3(10.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z }, rect(0.0, 0.0, 6.0, 4.0)),
        (Plane { origin: v3(0.0, 6.0, 0.0), x: Vec3::Z, y: Vec3::X }, rect(0.0, 0.0, 4.0, 10.0)),
        (Plane { origin: v3(0.0, 0.0, 4.0), x: Vec3::X, y: Vec3::Y }, rect(0.0, 0.0, 10.0, 6.0)),
    ];
    for (plane, face) in &faces {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    Ok(swept)
}
