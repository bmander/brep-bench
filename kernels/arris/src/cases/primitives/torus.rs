//! A solid torus about the z axis: its tube's middle circle of radius 10, the tube of radius 2.
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const MAJOR: f64 = 10.0;
const MINOR: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // workaround: no torus primitive
    // arris has no torus primitive: the tube's circle, drawn in the xz plane, turned about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let tube = Profile {
        plane: xz,
        outer: ProfileLoop::Circle { center: Point2::new(MAJOR, 0.0), radius: MINOR },
        holes: vec![],
    };
    let (torus, _) = revolve(m, &tube, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    Ok(torus)
}
