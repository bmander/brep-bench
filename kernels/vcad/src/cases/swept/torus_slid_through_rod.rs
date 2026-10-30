//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a torus slid
//! along z past it: the torus's axis parallel to y, the circle through its tube's centres of
//! radius 6, its tube of radius 2, its centre slid from (11.5, 0, −9) to (11.5, 0, 29).
//!
//! vcad has no sweep of a solid, so the wheel's sweep is built from its shape by elementary
//! geometry. The slide runs along z, in the torus's own mid-plane (y = 0). A line along z through
//! a point (x, y) meets the torus where (x, y) is in the torus's shadow along z: within 2 of the
//! segment from (5.5, 0) to (17.5, 0), the stadium the tube's circles cast. The line meets the
//! torus in at most two pieces, the gap between them at most the hole's width 8, and the slide of
//! 38 is longer, so the pieces' sweeps run together: on that line the wheel passes over everything
//! from below its first place to above its last. The sweep is the torus where it starts, the torus
//! where it ends, and between their centres the stadium extruded along z. The end tori lie wholly
//! below z = −1 and above z = 21, clear of the rod, so what the sweep takes from the rod is that
//! prism's alone: it is the tool cut here.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const ROD_RADIUS: f64 = 5.0;
const ROD_LENGTH: f64 = 20.0;
/// The wheel: its centre's x, the radius of the circle through its tube's centres and its tube's.
const WHEEL_X: f64 = 11.5;
const MAJOR: f64 = 6.0;
const MINOR: f64 = 2.0;
/// The wheel's centre's height where it starts and where it ends.
const START_Z: f64 = -9.0;
const END_Z: f64 = 29.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let rod = Solid::cylinder(ROD_RADIUS, ROD_LENGTH, SEGMENTS);
    // the wheel's shadow along z, sketched in the plane z = −9: the rectangle the tube's near and
    // far sections span, closed by a half disc of the tube's radius at each end
    let (near, far) = (WHEEL_X - MAJOR, WHEEL_X + MAJOR);
    let p = Point2::new;
    let line = |start, end| SketchSegment::Line { start, end };
    let arc = |start, end, center| SketchSegment::Arc { start, end, center, ccw: true };
    let shadow = vec![
        line(p(near, -MINOR), p(far, -MINOR)),
        arc(p(far, -MINOR), p(far, MINOR), p(far, 0.0)),
        line(p(far, MINOR), p(near, MINOR)),
        arc(p(near, MINOR), p(near, -MINOR), p(near, 0.0)),
    ];
    let shadow = SketchProfile::new(Point3::new(0.0, 0.0, START_Z), Vec3::x(), Vec3::y(), shadow)?;
    let swept = Solid::extrude(shadow, Vec3::new(0.0, 0.0, END_Z - START_Z))?;
    Ok(rod.try_difference(&swept)?)
}
