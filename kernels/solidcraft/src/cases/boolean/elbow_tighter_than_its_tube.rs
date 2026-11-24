//! The outside of a pipe elbow tighter than its tube: the disk of radius 20 about (0, 0, 18) in
//! the plane x = 0 turned a quarter turn about the y axis, +z towards +x, with a straight leg of
//! radius 20 and length 40 running back from each end of the turn.
//!
//! The disk reaches 2 past the axis, and no layer revolves a profile across its axis (the
//! kernel's `revolve` refuses one), so the turn is the disk's part z ≥ 0 turned: its part z < 0
//! turns inside the leg along −x. A point (0, y, z) of it, z < 0, turned by θ is
//! (z sin θ, y, z cos θ): x ≤ 0, and 18 − z cos θ ≤ 18 − z, so within the leg's radius of its
//! line y = 0, z = 18.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    // the plane x = 0, sketch x along +y and sketch y along +z: its normal is +x, the way a
    // partial turn goes
    let section = Plane { origin: Vec3::ZERO, x: Vec3::Y, y: Vec3::Z };
    // the disk's long arc from where it crosses the axis, at the angle −b about its centre, round
    // over its top to the other crossing, closed along the axis
    let b = (18.0f64 / 20.0).asin();
    let arc = Seg2::Arc { center: v2(0.0, 18.0), radius: 20.0, start: -b, sweep: PI + 2.0 * b };
    let close = Seg2::Line { a: arc.end(), b: arc.start() };
    let disk = Region2 { outer: Loop2 { segs: vec![arc, close] }, holes: vec![] };
    let turn = one(solidcraft_kernel::revolve(&section, &[disk], Vec2::ZERO, Vec2::X, FRAC_PI_2)?)?;
    let leg_x = solidcraft_kernel::cylinder(v3(-40.0, 0.0, 18.0), Vec3::X, 20.0, 40.0)?;
    let leg_z = zcyl(18.0, 0.0, 20.0, -40.0, 0.0)?;
    union(&union(&turn, &leg_x)?, &leg_z)
}
