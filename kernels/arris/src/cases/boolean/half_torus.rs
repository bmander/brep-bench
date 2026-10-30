//! A torus of radii 10 and 2 about the z axis, with everything where x < 0 cut away.
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, split, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const MAJOR: f64 = 10.0;
const MINOR: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the tube's circle in the xz plane, turned about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let tube = Profile {
        plane: xz,
        outer: ProfileLoop::Circle { center: Point2::new(MAJOR, 0.0), radius: MINOR },
        holes: vec![],
    };
    let (torus, _) = revolve(m, &tube, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    // split by the plane x = 0, keeping the side its normal +x points to
    let yz = Frame::from_z(Point3::origin(), Vec3::x())?;
    let halves = split(m, torus, &yz, &Control::NONE)?;
    Ok(halves.positive)
}
