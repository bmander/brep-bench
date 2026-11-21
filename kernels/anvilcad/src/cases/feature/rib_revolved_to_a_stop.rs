//! A hub of radius 10 on the z axis from z = 0 to 10, a stop (within 25 of the axis between 120°
//! and 135°) joined to it, and the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4 of the plane y = 0 turned
//! about z from 0° until it meets the stop's face at 120°, joined to them.
use anvil_brep::{Frame, Solid, revolve};
use glam::DVec2;

use crate::shared::{cylinder, e, rect, region, turn_z, union, xz};

pub fn build() -> Result<Solid, String> {
    // in the xz plane a negative turn about its sketch y (world z) runs from +x toward +y
    let turn = |r: (f64, f64), h: f64, degrees: f64| {
        revolve::revolve(&xz([0., 0., 0.]), &region(&[rect((r.0, 0.), (r.1, h))])?, DVec2::ZERO, DVec2::Y, -degrees.to_radians()).map_err(e)
    };
    let stop = turn((0., 25.), 10., 15.)?.transformed(&turn_z(120.));
    let body = union(&cylinder(&Frame::world(), 10., 0., 10.)?, &stop)?;
    union(&body, &turn((5., 20.), 4., 120.)?)
}
