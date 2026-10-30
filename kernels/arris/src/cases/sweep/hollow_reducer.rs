//! A hollow square tube along y from 0 to 40, tapering from 24 × 24 with a hole 20 × 20 to
//! 12 × 12 with a hole 8 × 8, each corner running straight from end to end.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{loft, LoftKind, OpError};
use arris::topo::{Body, Model};

const LENGTH: f64 = 40.0;
const WALL: f64 = 2.0;
/// The outside's half-width at y = 0 and at y = 40.
const START_HALF: f64 = 12.0;
const END_HALF: f64 = 6.0;

/// The square of half-width `h` about the y axis.
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

/// The hollow section at `y`: the outside of half-width `h`, the hole a wall in from it.
fn section(y: f64, h: f64) -> Result<Profile, OpError> {
    Ok(Profile {
        plane: Frame::new(Point3::new(0.0, y, 0.0), Vec3::y(), Vec3::x())?,
        outer: square(h),
        holes: vec![square(h - WALL)],
    })
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // a ruled loft joins the outsides corner to corner and the holes likewise: every long face a
    // flat trapezoid, and the hole a void of the one lump running through both caps
    let sections = [section(0.0, START_HALF)?, section(LENGTH, END_HALF)?];
    let (reducer, _) = loft(m, &sections, LoftKind::Ruled, false, &Control::NONE)?;
    Ok(reducer)
}
