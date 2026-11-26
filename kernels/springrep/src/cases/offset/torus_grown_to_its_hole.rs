//! The torus about the z axis of middle radius 4 and tube radius 2, grown by 2: the grown tube's
//! radius equals the middle circle's, a horn torus closing its hole at the origin.
use springrep::primitive::torus;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ring = torus(&at([0., 0., 0.]), 4., 2., "torus").map_err(e)?;
    grown(&ring, 2.)
}
