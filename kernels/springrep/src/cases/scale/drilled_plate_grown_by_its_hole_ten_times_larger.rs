//! `offset/drilled-plate-grown-by-its-hole` ten times larger: the plate from (−100, −100, 0) to
//! (100, 100, 40) with a hole of radius 30 drilled through it along the z axis, grown by 30, the
//! hole's radius: the hole closes onto its axis, and each end of it leaves a dimple, a horn
//! torus's inside, coming down to a cusp on the axis. Built as the original is, every length
//! times 10.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-100., -100., 0.]), Vec3::new(200., 200., 40.), "plate").map_err(e)?;
    let drill = cylinder(&at([0., 0., -10.]), 30., 60., "hole").map_err(e)?;
    let drilled = difference(&plate, &drill).map_err(e)?;
    grown(&drilled, 30.)
}
