//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3) grown
//! by 1: every point within 1 of it, its edges pieces of cylinders and its corners pieces of a
//! ball, SolidCraft's offset of the whole solid (`offset_solid`), its convex edges rounded. The
//! tetrahedron is SolidCraft's convex polyhedron of the four half-spaces its faces bound.
use crate::solids::*;
use solidcraft_kernel::{convex_polyhedron, HalfSpace};

pub fn build() -> R<Body> {
    // the face opposite corner c lies on the plane −c·x = 9, at √3 from the centre, facing away
    // from c
    let corners = [[3.0, 3.0, 3.0], [3.0, -3.0, -3.0], [-3.0, 3.0, -3.0], [-3.0, -3.0, 3.0]];
    let s = 27f64.sqrt();
    let faces: Vec<HalfSpace> =
        corners.iter().map(|c| HalfSpace { n: v3(-c[0] / s, -c[1] / s, -c[2] / s), d: 3f64.sqrt() }).collect();
    let tetrahedron = convex_polyhedron(&faces)?;
    solidcraft_kernel::offset_solid(&tetrahedron, 1.0, true)
}
