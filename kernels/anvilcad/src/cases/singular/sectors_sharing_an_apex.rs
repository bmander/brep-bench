//! The common part of two spherical sectors of a ball of radius 10 about the origin, each within
//! 45° of its axis: the first about +z, the second turned 60° about the y axis.
use anvil_brep::{Frame, Solid};
use glam::DVec3;

use crate::shared::{ball, cone, intersect};

pub fn build() -> Result<Solid, String> {
    // the sector: the ball within the cone from its apex at the origin, half angle 45°, past the
    // sphere
    let sector = intersect(&ball([0., 0., 0.], 10.)?, &cone(&Frame::world(), 0., 11., 11.)?)?;
    let (s, c) = 60f64.to_radians().sin_cos();
    let turn = Frame { origin: DVec3::ZERO, x: DVec3::new(c, 0., -s), y: DVec3::Y, z: DVec3::new(s, 0., c) };
    intersect(&sector, &sector.transformed(&turn))
}
