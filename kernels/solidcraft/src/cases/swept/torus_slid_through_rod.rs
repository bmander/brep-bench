//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a torus slid
//! along z past it: its axis parallel to y, the circle through its tube's centres of radius 6,
//! its tube of radius 2, its centre slid from (11.5, 0, −9) to (11.5, 0, 29).
//!
//! solidcraft-kernel has no sweep of a solid, so the wheel's sweep is built by elementary
//! geometry. A line along z through (x, y) meets the wheel if and only if (x, y) lies in its
//! shadow along z: within 2 of the segment from (5.5, 0) to (17.5, 0), a slot. The wheel is
//! symmetric about its centre's height, and on such a line it spans at most 16 with a gap of at
//! most 8, the hole, so slid 38 it passes over the whole of the line from its centre where it
//! starts to its centre where it ends. The sweep is then the slot extruded between those heights,
//! with the wheel where it starts and where it ends; those two lie wholly below z = −1 and above
//! z = 21, clear of the rod, so the slot's prism is what the sweep takes from the rod.
use crate::solids::*;

/// The shadow along z of a wheel whose axis runs along y through (`x`, `y`, `z`): its tube of
/// radius `tube` about a circle of radius `ring`. The slot within `tube` of the segment from
/// `x − ring` to `x + ring` at height `y`, on the plane at height `z`.
pub fn shadow(x: f64, y: f64, z: f64, ring: f64, tube: f64) -> (Plane, Region2) {
    use solidcraft_geom::Seg2;
    use std::f64::consts::{FRAC_PI_2, PI};
    let (near, far) = (x - ring, x + ring);
    let segs = vec![
        Seg2::Line { a: v2(near, y - tube), b: v2(far, y - tube) },
        Seg2::Arc { center: v2(far, y), radius: tube, start: -FRAC_PI_2, sweep: PI },
        Seg2::Line { a: v2(far, y + tube), b: v2(near, y + tube) },
        Seg2::Arc { center: v2(near, y), radius: tube, start: FRAC_PI_2, sweep: PI },
    ];
    let plane = Plane { origin: v3(0.0, 0.0, z), x: Vec3::X, y: Vec3::Y };
    (plane, Region2 { outer: Loop2 { segs }, holes: vec![] })
}

pub fn build() -> R<Body> {
    let rod = zcyl(0.0, 0.0, 5.0, 0.0, 20.0)?;
    let (plane, slot) = shadow(11.5, 0.0, -9.0, 6.0, 2.0);
    cut(&rod, &extrude(&plane, slot, 0.0, 38.0)?)
}
