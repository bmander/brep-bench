//! The plate from (−10, −10, 0) to (10, 10, 4) with a round recess of radius 3 sunk 1 into its top
//! about the z axis, grown by 4, past the recess's radius: springrep's own offset.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 4.), "plate").map_err(e)?;
    // the recess's cutter from its floor at z = 3 up past the top
    let recess = cylinder(&at([0., 0., 3.]), 3., 2., "recess").map_err(e)?;
    let recessed = difference(&plate, &recess).map_err(e)?;
    grown(&recessed, 4.)
}
