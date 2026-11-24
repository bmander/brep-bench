//! The rod of radius 5 about the z axis from z = 0 to z = 20 less everything a wheel passes
//! through as it slides along z from center (11.5, 0, −9) to (11.5, 0, 29): the torus with its
//! axis along y, its tube's centers 6 from it and its tube of radius 2.
//!
//! springrep's sweep of a solid declines this one (it leaves edges unpaired). The torus is
//! everything within 2 of its tube's center circle, and the slide, longer than the hole is wide,
//! carries that circle over a stadium: so the sweep is everything within 2 of the stadium, its
//! middle the stadium's section square to the slide carried along, and its ends half the ring with
//! its hole filled (`shared::swept::slid_ring`).
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::swept::{rod, slid_ring, v3};

pub fn build() -> Result<Brep, String> {
    let groove = slid_ring(v3(11.5, 0.0, -9.0), Vec3::Y, Vec3::Z, 6.0, 2.0, 38.0, "wheel")?;
    difference(&rod()?, &groove).map_err(e)
}
