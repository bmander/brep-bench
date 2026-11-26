//! The block (−20, −15, 0)–(20, 15, 10) with a pocket down to z = 4, the rectangle from (−10, −7)
//! to (10, 7) with its corners rounded to radius 2, grown by 3: springrep's own offset.
use std::f64::consts::FRAC_PI_2;

use springrep::boolean::difference;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::offset::grown;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([-20., -15., 0.]), Vec3::new(40., 30., 10.), "block").map_err(e)?;
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let outline = Profile::outline(vec![
        NamedSegment::line(p(-8., -7.), p(8., -7.), "wall"),
        NamedSegment::arc(p(8., -5.), 2., -FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(10., -5.), p(10., 5.), "wall"),
        NamedSegment::arc(p(8., 5.), 2., 0., FRAC_PI_2, "corner"),
        NamedSegment::line(p(8., 7.), p(-8., 7.), "wall"),
        NamedSegment::arc(p(-8., 5.), 2., FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(-10., 5.), p(-10., -5.), "wall"),
        NamedSegment::arc(p(-8., -5.), 2., 2. * FRAC_PI_2, FRAC_PI_2, "corner"),
    ]);
    // the pocket from its floor up past the block's top
    let pocket = extrude(&outline, &at([0., 0., 4.]), 7., "pocket").map_err(e)?;
    let pocketed = difference(&body, &pocket).map_err(e)?;
    grown(&pocketed, 3.)
}
