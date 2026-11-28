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
    slid(v3(8.0, 5.0, 3.0))
}

/// The box from the origin to (10, 6, 4) slid along `slide`, whose parts are none negative: the
/// box and each face it leads with, x = 10, y = 6 or z = 4 where `slide` has a part along x, y
/// or z, slid the whole way.
pub fn slid(slide: Vec3) -> R<Body> {
    let mut swept = block((0.0, 0.0, 0.0), (10.0, 6.0, 4.0))?;
    // each leading face on a plane whose normal is its outward one, its sides along the box's
    // edges from the corner nearest the origin
    let faces = [
        (slide.x, Plane { origin: v3(10.0, 0.0, 0.0), x: Vec3::Y, y: Vec3::Z }, rect(0.0, 0.0, 6.0, 4.0)),
        (slide.y, Plane { origin: v3(0.0, 6.0, 0.0), x: Vec3::Z, y: Vec3::X }, rect(0.0, 0.0, 4.0, 10.0)),
        (slide.z, Plane { origin: v3(0.0, 0.0, 4.0), x: Vec3::X, y: Vec3::Y }, rect(0.0, 0.0, 10.0, 6.0)),
    ];
    for (_, plane, face) in faces.iter().filter(|(along, ..)| *along > 0.0) {
        let path = [PathSeg::Line { a: plane.origin, b: plane.origin + slide }];
        swept = union(&swept, &sweep(plane, face, &path)?)?;
    }
    Ok(swept)
}
