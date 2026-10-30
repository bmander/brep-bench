//! A pipe of radius 1 along three turns of a helix of radius 10 and pitch 5, its ends flat.
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const HELIX_RADIUS: f64 = 10.0;
const PITCH: f64 = 5.0;
const TURNS: f64 = 3.0;
const PIPE_RADIUS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let start = Point3::new(HELIX_RADIUS, 0.0, 0.0);
    let helix = Path {
        start,
        segments: vec![PathSegment::Helix {
            axis: Axis::z_at(Point3::origin()),
            pitch: PITCH,
            angle: TURNS * TAU,
        }],
    };
    // the section square to the helix where it starts: its tangent there is (0, 2πr, pitch)
    let tangent = Vec3::new(0.0, TAU * HELIX_RADIUS, PITCH);
    let section = Profile {
        plane: Frame::new(start, tangent, Vec3::x())?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: PIPE_RADIUS },
        holes: vec![],
    };
    // a spring's frame: the binormal held to the axis, so the section turns with the helix
    let law = FrameLaw::FixedBinormal(Vec3::z_axis());
    let (pipe, _) = sweep(m, &section, &helix, law, &Control::NONE)?;
    Ok(pipe)
}
