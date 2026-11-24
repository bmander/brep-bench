//! A disc of radius 100, flat beneath, its rim 3 high and its top crowned by the sphere of radius
//! 800 through the rim's top, joined to a vane, the obround within 2 of the segment from (22, 0) to
//! (88, 0) standing from z = 1 to 31; the loop where the vane's sides meet the crown rounded to
//! radius 2 by a rolling ball.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::chamfer::turned;
use crate::shared::{arc_to, at, e};

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    // the sphere's centre on the axis below, 800 from the rim's top circle
    let c = 3. - 100. * 63f64.sqrt();
    let disc = turned(
        vec![
            NamedSegment::line(p(0., 0.), p(100., 0.), "bottom"),
            NamedSegment::line(p(100., 0.), p(100., 3.), "rim"),
            arc_to(p(0., c), p(100., 3.), p(0., c + 800.), true, "crown"),
            NamedSegment::line(p(0., c + 800.), p(0., 0.), "axis"),
        ],
        "disc",
    )?;
    let vane = Profile::outline(vec![
        NamedSegment::line(p(22., -2.), p(88., -2.), "side"),
        arc_to(p(88., 0.), p(88., -2.), p(88., 2.), true, "side"),
        NamedSegment::line(p(88., 2.), p(22., 2.), "side"),
        arc_to(p(22., 0.), p(22., 2.), p(22., -2.), true, "side"),
    ]);
    let vane = extrude(&vane, &at([0., 0., 1.]), 30., "vane").map_err(e)?;
    let body = union(&disc, &vane).map_err(e)?;
    fillet(&body, &[("disc.crown", "vane.side")], 2., "round").map_err(e)
}
