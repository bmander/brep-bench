//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two overlapping beads slid 40 along z past it. The beads are tori on one axis parallel to y:
//! one centred at (11.5, 0, −9) with radii 6 and 2, the other 1.5 further along the axis, at
//! (11.5, 1.5, −9), with radii 6 and 1.5.
//!
//! vcad has no sweep of a solid, so the wheel's sweep is built from its shape by elementary
//! geometry. The sweep of a union is the union of its parts' sweeps, so each bead is swept alone.
//! A bead slides along z, in its own mid-plane, and a line along z through a point of its shadow
//! meets it in at most two pieces, the gap between them at most its hole's width 8 (or 9), shorter
//! than the slide of 40: on that line the bead passes over everything from below its first place
//! to above its last. So a bead's sweep is the bead where it starts, the bead where it ends, and
//! between their centres its shadow along z extruded: the stadium within its tube's radius of the
//! segment its tube's centres cast, from x = 5.5 to x = 17.5. The beads where they start lie wholly
//! below z = −1 and where they end wholly above z = 21, clear of the rod, so what the wheel takes
//! from the rod is the two prisms': each is cut in turn.
use crate::solids::SEGMENTS;
use vcad_kernel::vcad_kernel_math::{Point2, Point3, Vec3};
use vcad_kernel::vcad_kernel_sketch::{SketchError, SketchProfile, SketchSegment};
use vcad_kernel::Solid;

const ROD_RADIUS: f64 = 5.0;
const ROD_LENGTH: f64 = 20.0;
/// The wheel's centre's x and the radius of the circle through each bead's tube's centres.
const WHEEL_X: f64 = 11.5;
const MAJOR: f64 = 6.0;
/// Each bead: its centre's y and its tube's radius.
const BEADS: [(f64, f64); 2] = [(0.0, 2.0), (1.5, 1.5)];
/// The wheel's centre's height where it starts, and how far it slides.
const START_Z: f64 = -9.0;
const SLIDE: f64 = 40.0;

/// The prism a bead sweeps between its places: its shadow along z, sketched in the plane
/// z = −9, extruded along the slide.
fn swept(y: f64, minor: f64) -> Result<Solid, SketchError> {
    let (near, far) = (WHEEL_X - MAJOR, WHEEL_X + MAJOR);
    let p = Point2::new;
    let line = |start, end| SketchSegment::Line { start, end };
    let arc = |start, end, center| SketchSegment::Arc { start, end, center, ccw: true };
    let shadow = vec![
        line(p(near, y - minor), p(far, y - minor)),
        arc(p(far, y - minor), p(far, y + minor), p(far, y)),
        line(p(far, y + minor), p(near, y + minor)),
        arc(p(near, y + minor), p(near, y - minor), p(near, y)),
    ];
    let shadow = SketchProfile::new(Point3::new(0.0, 0.0, START_Z), Vec3::x(), Vec3::y(), shadow)?;
    Solid::extrude(shadow, Vec3::new(0.0, 0.0, SLIDE))
}

// workaround: no sweep of a solid
pub fn build() -> Result<Solid, crate::Refusal> {
    let mut rod = Solid::cylinder(ROD_RADIUS, ROD_LENGTH, SEGMENTS);
    for (y, minor) in BEADS {
        rod = rod.try_difference(&swept(y, minor)?)?;
    }
    Ok(rod)
}
