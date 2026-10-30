//! A rectangle in the xz plane, x from 6 to 10 and z from 0 to 4, turned 90° about z.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_PI_2;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;
const ANGLE: f64 = FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane: u along x, v along z, its normal -y so a positive turn about z leaves it
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    let rectangle = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(INNER, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(OUTER, 0.0)),
                ProfileSegment::LineTo(p(OUTER, HEIGHT)),
                ProfileSegment::LineTo(p(INNER, HEIGHT)),
                ProfileSegment::LineTo(p(INNER, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (sector, _) = revolve(m, &rectangle, Axis::z_at(Point3::origin()), ANGLE, &Control::NONE)?;
    Ok(sector)
}
