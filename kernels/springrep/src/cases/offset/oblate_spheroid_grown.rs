//! The oblate spheroid (x² + y²)/36 + z²/9 ≤ 1 of `geometry/oblate-spheroid`, grown by 1:
//! springrep's own offset.
use springrep::topo::Brep;

use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::offset::grown;

pub fn build() -> Result<Brep, String> {
    grown(&oblate_spheroid::build()?, 1.)
}
