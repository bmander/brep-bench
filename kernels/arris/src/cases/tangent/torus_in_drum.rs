//! A torus about the z axis, its tube's middle circle of radius 10 and the tube of radius 3,
//! united with a drum of radius 13 on the z axis from z = −1 to 1 that touches its outer equator.
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{fuse, primitive_cylinder, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const MAJOR: f64 = 10.0;
const MINOR: f64 = 3.0;
const DRUM_RADIUS: f64 = 13.0;
const DRUM_HALF_HEIGHT: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // arris has no torus primitive: the tube's circle, drawn in the xz plane, turned about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let tube = Profile {
        plane: xz,
        outer: ProfileLoop::Circle { center: Point2::new(MAJOR, 0.0), radius: MINOR },
        holes: vec![],
    };
    let (torus, _) = revolve(m, &tube, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    let base = Point3::new(0.0, 0.0, -DRUM_HALF_HEIGHT);
    let (drum, _) = primitive_cylinder(
        m,
        Axis::z_at(base),
        DRUM_RADIUS,
        2.0 * DRUM_HALF_HEIGHT,
        &Control::NONE,
    )?;
    let (joined, _) = fuse(m, torus, drum, &Control::NONE)?;
    Ok(joined)
}
