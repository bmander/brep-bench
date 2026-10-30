//! A rectangle in the xz plane, x from 6 to 10 and z from 0 to 4, turned a full turn about z.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const INNER: f64 = 6.0;
const OUTER: f64 = 10.0;
const HEIGHT: f64 = 4.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane: u along x, v along z
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
    let (washer, _) = revolve(m, &rectangle, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    Ok(washer)
}
