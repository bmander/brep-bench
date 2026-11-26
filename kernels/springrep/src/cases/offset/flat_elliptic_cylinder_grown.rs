//! The ellipse x²/100 + y²/4 ≤ 1 extruded from z = 0 to z = 10, grown by 1, as
//! `offset/elliptic-cylinder-grown` grows its 5 × 3 ellipse: springrep's own offset.
use springrep::boolean::unify;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let section = Profile::outline(ellipse(Vec2::ZERO, 10., 2., "side"));
    let drum = extrude(&section, &at([0., 0., 0.]), 10., "drum").map_err(e)?;
    // the ellipse is drawn as two halves; unified, the side and each rim are one face and one edge
    let drum = unify(&drum).map_err(e)?;
    grown(&drum, 1.)
}
