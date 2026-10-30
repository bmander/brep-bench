//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel of
//! two overlapping beads slid 40 along z past it. The beads are tori on one axis parallel to y:
//! one centred at (11.5, 0, −9) with radii 6 and 2, the other 1.5 further along the axis, at
//! (11.5, 1.5, −9), with radii 6 and 1.5.
//!
//! arris has no sweep of a solid, so the wheel's sweep is built from its shape by elementary
//! geometry. The sweep of a union is the union of its parts' sweeps, so each bead is swept alone.
//! A bead slides along z, in its own mid-plane, and a line along z through a point of its shadow
//! meets it in at most two pieces, the gap between them at most its hole's width 8 (or 9), shorter
//! than the slide of 40: on that line the bead passes over everything from below its first place
//! to above its last. So a bead's sweep is the bead where it starts, the bead where it ends, and
//! between their centres its shadow along z extruded: the stadium within its tube's radius of the
//! segment its tube's centres cast, from x = 5.5 to x = 17.5. The beads where they start lie wholly
//! below z = −1 and where they end wholly above z = 21, clear of the rod, so what the wheel takes
//! from the rod is the two prisms', fused into the wheel's shadow extruded.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, extrude, fuse, primitive_cylinder, OpError};
use arris::topo::{Body, Model};

const ROD_RADIUS: f64 = 5.0;
const ROD_LENGTH: f64 = 20.0;
/// The wheel's axis: its x and its height where it starts.
const AXIS_X: f64 = 11.5;
const START_Z: f64 = -9.0;
const SLIDE: f64 = 40.0;
/// Each bead: how far along the axis (y) its centre is, the radius of the circle through its
/// tube's centres, and its tube's.
const BEADS: [(f64, f64, f64); 2] = [(0.0, 6.0, 2.0), (1.5, 6.0, 1.5)];

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let axis = Axis::z_at(Point3::origin());
    let (rod, _) = primitive_cylinder(m, axis, ROD_RADIUS, ROD_LENGTH, &Control::NONE)?;
    // the plane z = −9 where the wheel starts: u along x, v along y
    let start = Frame::new(Point3::new(0.0, 0.0, START_Z), Vec3::z(), Vec3::x())?;
    let p = Point2::new;
    let mut prisms = Vec::new();
    for (along, major, minor) in BEADS {
        // the bead's shadow along z: the rectangle its tube's near and far sections span, closed
        // by a half disc of the tube's radius at each end
        let (near, far) = (AXIS_X - major, AXIS_X + major);
        let shadow = Profile {
            plane: start,
            outer: ProfileLoop::Path {
                start: p(near, along - minor),
                segments: vec![
                    ProfileSegment::LineTo(p(far, along - minor)),
                    ProfileSegment::ArcTo {
                        to: p(far, along + minor),
                        via: p(far + minor, along),
                    },
                    ProfileSegment::LineTo(p(near, along + minor)),
                    ProfileSegment::ArcTo {
                        to: p(near, along - minor),
                        via: p(near - minor, along),
                    },
                ],
            },
            holes: vec![],
        };
        prisms.push(extrude(m, &shadow, Vec3::z(), SLIDE, &Control::NONE)?.0);
    }
    let (swept, _) = fuse(m, prisms[0], prisms[1], &Control::NONE)?;
    let (grooved, _) = cut(m, rod, swept, &Control::NONE)?;
    Ok(grooved)
}
