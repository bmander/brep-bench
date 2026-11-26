//! A pipe of radius 1 along forty right-handed turns of the helix of radius 10 about the z axis,
//! rising 5 a turn from (10, 0, 0), its section square to the path, its ends flat, as
//! `sweep/helical-pipe` makes its three turns: the disk square to the helix at its start, screwed
//! along it.
use std::f64::consts::PI;

use springrep::math::{Frame, Point3, Vec2, Vec3};
use springrep::profile::Profile;
use springrep::sweep::{Helix, helix};
use springrep::topo::Brep;

use crate::shared::e;

pub fn build() -> Result<Brep, String> {
    let screw = Helix {
        origin: Point3::ZERO,
        axis: Vec3::Z,
        rise: 5. / (2. * PI),
        sign: 1.,
    };
    // the helix's tangent at its start, (10, 0, 0)
    let tangent = Vec3::new(0., 10., screw.rise);
    let section = Frame::new(Point3::new(10., 0., 0.), tangent, Vec3::X).map_err(e)?;
    let disk = Profile::circle(Vec2::ZERO, 2.);
    helix(&disk, &section, &screw, 80. * PI, "pipe").map_err(e)
}
