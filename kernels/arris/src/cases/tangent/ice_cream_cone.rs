//! A ball of radius 15 on the origin united with a solid cone on the z axis, its apex at
//! (0, 0, −25) and its base the disc of radius 12 at z = −9, its side tangent to the sphere there.
use crate::solids::ball;
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{fuse, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const BALL_RADIUS: f64 = 15.0;
const APEX_Z: f64 = -25.0;
const BASE_Z: f64 = -9.0;
const BASE_RADIUS: f64 = 12.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (scoop, _) = ball(m, Point3::origin(), BALL_RADIUS, &Control::NONE)?;
    // arris has no cone primitive: the right triangle under its slant in the xz plane, u along x
    // and v along z, one leg on the axis from the apex to the base, turned a full turn about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    let triangle = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(0.0, APEX_Z),
            segments: vec![
                ProfileSegment::LineTo(p(BASE_RADIUS, BASE_Z)),
                ProfileSegment::LineTo(p(0.0, BASE_Z)),
                ProfileSegment::LineTo(p(0.0, APEX_Z)),
            ],
        },
        holes: vec![],
    };
    let (cone, _) = revolve(m, &triangle, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    let (joined, _) = fuse(m, scoop, cone, &Control::NONE)?;
    Ok(joined)
}
