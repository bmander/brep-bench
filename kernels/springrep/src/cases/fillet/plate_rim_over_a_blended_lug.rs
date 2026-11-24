//! A plate (−40, −25, 0)–(40, 25, 13) with its upright edges rounded to radius 10, joined to a lug,
//! the cylinder of radius 9 about the upright through (0, 25); the two concave edges where the lug
//! meets the plate's north side rounded to radius 8, then the rim of the bottom, over the lug and
//! its blends, rounded to radius 2 as one round.
use std::f64::consts::FRAC_PI_2;

use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let outline = Profile::outline(vec![
        NamedSegment::line(p(-30., -25.), p(30., -25.), "south"),
        NamedSegment::arc(p(30., -15.), 10., -FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(40., -15.), p(40., 15.), "east"),
        NamedSegment::arc(p(30., 15.), 10., 0., FRAC_PI_2, "corner"),
        NamedSegment::line(p(30., 25.), p(-30., 25.), "north"),
        NamedSegment::arc(p(-30., 15.), 10., FRAC_PI_2, FRAC_PI_2, "corner"),
        NamedSegment::line(p(-40., 15.), p(-40., -15.), "west"),
        NamedSegment::arc(p(-30., -15.), 10., 2. * FRAC_PI_2, FRAC_PI_2, "corner"),
    ]);
    let plate = extrude(&outline, &at([0., 0., 0.]), 13., "plate").map_err(e)?;
    let lug = cylinder(&at([0., 25., 0.]), 9., 13., "lug").map_err(e)?;
    let body = union(&plate, &lug).map_err(e)?;
    let body = fillet(&body, &[("plate.north", "lug.side")], 8., "blend").map_err(e)?;
    // one edge of the bottom's rim picks the whole tangent chain
    fillet(&body, &[("plate.start", "plate.south")], 2., "round").map_err(e)
}
