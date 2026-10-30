//! A hollow square duct, 18 × 18 with a hole 14 × 14, swept a quarter turn round an arc of
//! radius 30 about the z axis, its section in the plane through the axis.
use arris::geom::{FrameLaw, Path, PathSegment, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_1_SQRT_2;

const BEND_RADIUS: f64 = 30.0;
const OUTER_HALF: f64 = 9.0;
const INNER_HALF: f64 = 7.0;

/// The square of half-width `h` about the section's centre.
fn square(h: f64) -> ProfileLoop {
    let p = Point2::new;
    ProfileLoop::Path {
        start: p(h, h),
        segments: vec![
            ProfileSegment::LineTo(p(-h, h)),
            ProfileSegment::LineTo(p(-h, -h)),
            ProfileSegment::LineTo(p(h, -h)),
            ProfileSegment::LineTo(p(h, h)),
        ],
    }
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let start = Point3::new(BEND_RADIUS, 0.0, 0.0);
    let arc = Path {
        start,
        segments: vec![PathSegment::ArcTo {
            via: Point3::new(BEND_RADIUS * FRAC_1_SQRT_2, BEND_RADIUS * FRAC_1_SQRT_2, 0.0),
            to: Point3::new(0.0, BEND_RADIUS, 0.0),
        }],
    };
    // the hollow square in the xz plane at x = 30, square to the arc, whose tangent there is +y
    let section = Profile {
        plane: Frame::new(start, Vec3::y(), Vec3::x())?,
        outer: square(OUTER_HALF),
        holes: vec![square(INNER_HALF)],
    };
    // the section's plane holds the arc's axis, so arris sweeps it exactly: planes and cylinders
    let (duct, _) = sweep(m, &section, &arc, FrameLaw::RotationMinimizing, &Control::NONE)?;
    Ok(duct)
}
