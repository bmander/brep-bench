//! The ring sector (radii 8 to 12, heights −1 to 1, angles 0° to 60°) less everything a ring passes
//! through as it turns about the z axis from −140° to 30°: the torus about the upright line through
//! (10, 0), its tube's centers on the circle of radius 1 in the plane z = 0, its tube of radius 0.4.
use springrep::boolean::difference;
use springrep::primitive::torus;
use springrep::sweep::solid;
use springrep::topo::Brep;

use crate::shared::swept::{deg, sector_block, turning};
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([10., 0., 0.]), 1., 0.4, "ring").map_err(e)?;
    let notch = solid(&ring, &turning(deg(-140.), deg(30.)), "notch").map_err(e)?;
    difference(&sector_block()?, &notch).map_err(e)
}
