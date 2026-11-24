//! The disc of radius 100 from z = −20 to 20, counterbored to radius 60 from z = 0 up; a hole of
//! radius 3.5 about the vertical line through (59.9, 0), 0.1 inside the counterbore's wall, from
//! z = −15 up to the floor, and its 90° countersink, of radius 4 on the floor, cut away. Both reach
//! under the wall.
use crate::solids::*;
use std::f64::consts::TAU;

pub fn build() -> R<Body> {
    let disc = zcyl(0.0, 0.0, 100.0, -20.0, 20.0)?;
    // the counterbore's cutter runs 1 past the top
    let bored = cut(&disc, &zcyl(0.0, 0.0, 60.0, 0.0, 21.0)?)?;
    // the hole's and the countersink's cutters end on the floor's plane, as the case says: above
    // it, beyond ρ = 60, is the wall's material
    let holed = cut(&bored, &zcyl(59.9, 0.0, 3.5, -15.0, 0.0)?)?;
    // solidcraft-kernel has no cone: the sink is its half section, the right triangle from its
    // point 4 below the floor to its circle of radius 4 on it, turned about the hole's axis
    let section = Plane { origin: v3(59.9, 0.0, 0.0), x: Vec3::X, y: Vec3::Z };
    let sink = one(solidcraft_kernel::revolve(&section, &[polygon(&[(0.0, -4.0), (4.0, 0.0), (0.0, 0.0)])], Vec2::ZERO, Vec2::Y, TAU)?)?;
    cut(&holed, &sink)
}
