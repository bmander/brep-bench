//! SolidCraft's Shell (`shell`) hollows a body inward to a wall of the thickness, opening the faces
//! picked by points on them.
use crate::solids::*;
use solidcraft_geom::Seg2;
use std::f64::consts::PI;

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    // the half section in x and z: the side up to z = 10, then the quarter circle of radius 5
    // about (0, 10) over to the axis, turned about z (the union of a cylinder and a ball tangent
    // to it along its rim is refused)
    let section = Loop2 {
        segs: vec![
            line((0.0, 0.0), (5.0, 0.0)),
            line((5.0, 0.0), (5.0, 10.0)),
            Seg2::Arc { center: v2(0.0, 10.0), radius: 5.0, start: 0.0, sweep: PI / 2.0 },
            line((0.0, 15.0), (0.0, 0.0)),
        ],
    };
    let capsule = revolve_z(Region2 { outer: section, holes: vec![] }, 360.0)?;
    solidcraft_kernel::shell(&capsule, &[v3(0.0, 3.0, 14.0)], 1.0)
}
