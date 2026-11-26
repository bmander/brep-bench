//! The plate from (−30, −30, 0) to (30, 30, 4) with a hole of radius 20 drilled through it along
//! the z axis, grown by 20, the hole's radius: the hole closes onto its axis, each end a dimple.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-30., -30., 0.]), Vec3::new(60., 60., 4.), "plate").map_err(e)?;
    let drill = cylinder(&at([0., 0., -1.]), 20., 6., "hole").map_err(e)?;
    let drilled = difference(&plate, &drill).map_err(e)?;
    grown(&drilled, 20.)
}
