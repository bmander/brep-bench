//! A thin blade from z = 0 to z = 60: the rectangle 40 × 0.4 centred on the z axis, turned
//! steadily about it, right-handed, a whole turn on the way up, its corners on four helices.
//!
//! Every section is the same rectangle, so the blade is the rectangle carried by the screw motion
//! about the z axis that rises 60 in a turn: a sweep along a helix about that axis of that pitch,
//! with the binormal held to the axis, as a spring's is, so the rectangle turns with the helix
//! and stays level. Every point of the rectangle runs along such a helix, its corners along the
//! four guides; the sweep follows the one through the middle of a broad side, (0, 0.2, 0).
//! (Along a corner's own helix arris refuses: the rectangle reaches 40 across the axis, past that
//! helix's radius of curvature, 24.6, which its fold test reads as the swept surface folding,
//! though a level section carried by a screw motion never does.)
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const HEIGHT: f64 = 60.0;
const HALF_LENGTH: f64 = 20.0;
const HALF_WIDTH: f64 = 0.2;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (a, b) = (HALF_LENGTH, HALF_WIDTH);
    // the helix through the middle of a broad side, (0, b, 0), rising HEIGHT in its one turn
    let guide = Path {
        start: Point3::new(0.0, b, 0.0),
        segments: vec![PathSegment::Helix {
            axis: Axis::z_at(Point3::origin()),
            pitch: HEIGHT,
            angle: TAU,
        }],
    };
    // the rectangle at the foot, level; a sweep takes its profile where it stands
    let p = Point2::new;
    let foot = Profile {
        plane: Frame::new(Point3::origin(), Vec3::z(), Vec3::x())?,
        outer: ProfileLoop::Path {
            start: p(a, b),
            segments: vec![
                ProfileSegment::LineTo(p(-a, b)),
                ProfileSegment::LineTo(p(-a, -b)),
                ProfileSegment::LineTo(p(a, -b)),
                ProfileSegment::LineTo(p(a, b)),
            ],
        },
        holes: vec![],
    };
    // the binormal held to the axis: the frame at each point of the helix is the first one turned
    // about the axis and raised, so the rectangle is carried by the screw motion itself
    let law = FrameLaw::FixedBinormal(Vec3::z_axis());
    let (blade, _) = sweep(m, &foot, &guide, law, &Control::NONE)?;
    Ok(blade)
}
