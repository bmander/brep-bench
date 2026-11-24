//! A dog-bone plate 10 thick, its lobes the circles of radius 10 about (±30, 0) joined by a waist
//! between y = ±4 through concave blends of radius 6, joined to a boss: the cylinder of radius 10
//! about the lobe at (30, 0), from z = 0 to 30, so that the boss's side and the lobe's are one.
use springrep::boolean::union;
use springrep::math::Vec2;
use springrep::primitive::cylinder;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{arc_to, at, e};

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    // each blend's centre is 6 off its side, at y = ±10, and 16 from its lobe's centre
    let xb = 30. - 2. * 39f64.sqrt();
    // where the blend about (x, ±10) touches the lobe about (±30, 0): 10 along the line between
    let touch = |lobe: Vec2, blend: Vec2| lobe + (blend - lobe) * (10. / 16.);
    let (east, west) = (p(30., 0.), p(-30., 0.));
    let (ne, nw, sw, se) = (p(xb, 10.), p(-xb, 10.), p(-xb, -10.), p(xb, -10.));
    let outline = Profile::outline(vec![
        arc_to(east, touch(east, se), touch(east, ne), true, "east"),
        arc_to(ne, touch(east, ne), p(xb, 4.), false, "blend"),
        NamedSegment::line(p(xb, 4.), p(-xb, 4.), "north"),
        arc_to(nw, p(-xb, 4.), touch(west, nw), false, "blend"),
        arc_to(west, touch(west, nw), touch(west, sw), true, "west"),
        arc_to(sw, touch(west, sw), p(-xb, -4.), false, "blend"),
        NamedSegment::line(p(-xb, -4.), p(xb, -4.), "south"),
        arc_to(se, p(xb, -4.), touch(east, se), false, "blend"),
    ]);
    let plate = extrude(&outline, &at([0., 0., 0.]), 10., "plate").map_err(e)?;
    let boss = cylinder(&at([30., 0., 0.]), 10., 30., "boss").map_err(e)?;
    union(&plate, &boss).map_err(e)
}
