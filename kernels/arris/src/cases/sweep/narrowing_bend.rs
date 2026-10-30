//! A hollow square duct round a quarter arc of radius 30 about the z axis, its outside's
//! half-width running linearly from 9 to 6 with the angle turned, its wall 2 thick.
use arris::geom::{MAX_SMOOTH_SECTIONS, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{loft, LoftKind, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_PI_2;

const BEND_RADIUS: f64 = 30.0;
const TURN: f64 = FRAC_PI_2;
const WALL: f64 = 2.0;
/// The outside's half-width at the start and at the end of the turn.
const START_HALF: f64 = 9.0;
const END_HALF: f64 = 6.0;

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

/// The hollow section an angle `theta` round the arc, in the plane through the z axis there.
fn section(theta: f64) -> Result<Profile, OpError> {
    let h = START_HALF + (END_HALF - START_HALF) * theta / TURN;
    let (s, c) = theta.sin_cos();
    let centre = Point3::new(BEND_RADIUS * c, BEND_RADIUS * s, 0.0);
    // square to the arc: its normal the arc's tangent, its u the radial direction
    let plane = Frame::new(centre, Vec3::new(-s, c, 0.0), Vec3::new(c, s, 0.0))?;
    Ok(Profile { plane, outer: square(h), holes: vec![square(h - WALL)] })
}

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // workaround: no sweep with a scaling law
    // arris's sweep carries its profile rigidly, with no law to scale it, so the narrowing duct
    // is a smooth loft through as many of its sections as one smooth loft takes, evenly round
    // the turn: one surface per side through all of them
    let last = MAX_SMOOTH_SECTIONS - 1;
    let sections = (0..=last)
        .map(|k| section(TURN * k as f64 / last as f64))
        .collect::<Result<Vec<_>, _>>()?;
    let (bend, _) = loft(m, &sections, LoftKind::Smooth, false, &Control::NONE)?;
    Ok(bend)
}
