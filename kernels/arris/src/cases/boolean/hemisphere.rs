//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away.
use crate::solids::ball;
use arris::math::{Control, Frame, Point3, Vec3};
use arris::ops::{split, OpError};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    // split by the xy plane, keeping the side its normal +z points to
    let xy = Frame::from_z(Point3::origin(), Vec3::z())?;
    let halves = split(m, sphere, &xy, &Control::NONE)?;
    Ok(halves.positive)
}
