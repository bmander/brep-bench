//! A circle of radius 2 swept square to a quarter arc of radius 10, its ends flat.
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_1_SQRT_2;

const ARC_RADIUS: f64 = 10.0;
const SECTION_RADIUS: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let start = Point3::new(ARC_RADIUS, 0.0, 0.0);
    let arc = Path {
        start,
        segments: vec![PathSegment::ArcTo {
            via: Point3::new(ARC_RADIUS * FRAC_1_SQRT_2, ARC_RADIUS * FRAC_1_SQRT_2, 0.0),
            to: Point3::new(0.0, ARC_RADIUS, 0.0),
        }],
    };
    // the section square to the arc where it starts, whose tangent there is +y
    let section = Profile {
        plane: Frame::new(start, Vec3::y(), Vec3::x())?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: SECTION_RADIUS },
        holes: vec![],
    };
    let (pipe, _) = sweep(m, &section, &arc, FrameLaw::RotationMinimizing, &Control::NONE)?;
    Ok(pipe)
}
