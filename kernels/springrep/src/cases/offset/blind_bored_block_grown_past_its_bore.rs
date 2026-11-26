//! The block from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 3 down its z axis from
//! the top to a flat floor at z = 2, grown by 4, past the bore's radius: springrep's own offset.
use springrep::boolean::difference;
use springrep::math::Vec3;
use springrep::primitive::{block, cylinder};
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let b = block(&at([-10., -10., 0.]), Vec3::new(20., 20., 10.), "block").map_err(e)?;
    // the bore's cutter from its floor at z = 2 up past the top
    let bore = cylinder(&at([0., 0., 2.]), 3., 9., "bore").map_err(e)?;
    let bored = difference(&b, &bore).map_err(e)?;
    grown(&bored, 4.)
}
