//! A block (−40, −30, 0)–(40, 30, 16) with a pocket cut down to z = 6, the rectangle from
//! (−25, −15) to (25, 15) with its corners rounded to radius 4; the rim of the pocket's floor
//! rounded to radius 4, the corners' own, filling, as one round.
use std::f64::consts::FRAC_PI_2;

use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([-40., -30., 0.]), Vec3::new(80., 60., 16.), "block").map_err(e)?;
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let outline = Profile::outline(vec![
        NamedSegment::line(p(-21., -15.), p(21., -15.), "wall"),
        NamedSegment::arc(p(21., -11.), 4., -FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(25., -11.), p(25., 11.), "wall"),
        NamedSegment::arc(p(21., 11.), 4., 0., FRAC_PI_2, "corner"),
        NamedSegment::line(p(21., 15.), p(-21., 15.), "wall"),
        NamedSegment::arc(p(-21., 11.), 4., FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(-25., 11.), p(-25., -11.), "wall"),
        NamedSegment::arc(p(-21., -11.), 4., 2. * FRAC_PI_2, FRAC_PI_2, "corner"),
    ]);
    // the pocket from its floor up past the block's top
    let pocket = extrude(&outline, &at([0., 0., 6.]), 11., "pocket").map_err(e)?;
    let body = difference(&body, &pocket).map_err(e)?;
    // one edge of the floor's rim picks the whole tangent chain
    fillet(&body, &[("pocket.start", "pocket.wall")], 4., "round").map_err(e)
}
