//! A circle of radius 2 swept along 350° of the circle of radius 10 about the z axis, from
//! (10, 0, 0) anticlockwise, kept square to the path, as `sweep/circle-along-arc` sweeps its quarter
//! turn.
use springrep::math::{Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Leg, along};
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section starts square to the path, heading along +y
    let start = frame([10., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let section = Profile::circle(Vec2::ZERO, 4.);
    // a bend about +z turns the heading +y toward z × y = −x, about the origin
    let bend = Leg::Arc { radius: 10., angle: 350f64.to_radians(), axis: Vec3::Z };
    along(&section, &start, &[bend], "pipe").map_err(e)
}
