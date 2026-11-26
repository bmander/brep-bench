//! `singular/sectors-sharing-an-apex` ten times larger: the common part of two spherical sectors of
//! a ball of radius 100 about the origin, each within 45° of its axis: one about +z, the other
//! about (sin 60°, 0, cos 60°). Built as the original is, every length times 10.
use std::f64::consts::{FRAC_PI_4, TAU};

use springrep::boolean::intersection;
use springrep::math::{Frame, Vec2, Vec3};
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::revolve;
use springrep::topo::Brep;

use crate::shared::{e, frame};

/// The sector about `axis`: its meridian, the cone's ruling out from the apex, the arc of the
/// sphere up to the axis and the axis back down, turned a whole turn.
fn sector(axis: [f64; 3]) -> Result<Brep, String> {
    let z = Vec3::from_array(axis);
    let up = Frame::from_axis(Vec3::ZERO, z).map_err(e)?;
    // the meridian plane holds the axis: sketch x out from it, sketch y along it
    let plane = frame([0., 0., 0.], (-up.y).to_array(), up.x.to_array())?;
    let rim = Vec2::new(100. * FRAC_PI_4.sin(), 100. * FRAC_PI_4.cos());
    let meridian = Profile::outline(vec![
        NamedSegment::line(Vec2::ZERO, rim, "cone"),
        NamedSegment::arc(Vec2::ZERO, 100., FRAC_PI_4, FRAC_PI_4, "cap"),
        NamedSegment::line(Vec2::new(0., 100.), Vec2::ZERO, "axis"),
    ]);
    revolve(&meridian, &plane, Vec3::ZERO, z, TAU, "sector").map_err(e)
}

pub fn build() -> Result<Brep, String> {
    let a = sector([0., 0., 1.])?;
    let s = 60f64.to_radians();
    let b = sector([s.sin(), 0., s.cos()])?;
    intersection(&a, &b).map_err(e)
}
