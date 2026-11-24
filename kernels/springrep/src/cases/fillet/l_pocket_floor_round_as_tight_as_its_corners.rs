//! A block (−20, −20, 0)–(20, 20, 8) with an L-shaped pocket cut down to z = 3, the L through
//! (−12, −12), (12, −12), (12, 0), (0, 0), (0, 12), (−12, 12) with its convex corners rounded to
//! radius 1 and its re-entrant corner to radius 3; the rim of the pocket's floor rounded to
//! radius 1, the convex corners' own, filling, as one round.
use springrep::blend::fillet;
use springrep::boolean::difference;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{arc_to, at, e};

pub fn build() -> Result<Brep, String> {
    let body = block(&at([-20., -20., 0.]), Vec3::new(40., 40., 8.), "block").map_err(e)?;
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let line = |a, b| NamedSegment::line(a, b, "wall");
    let corner = |c, a, b| arc_to(c, a, b, true, "corner");
    let outline = Profile::outline(vec![
        line(p(-11., -12.), p(11., -12.)),
        corner(p(11., -11.), p(11., -12.), p(12., -11.)),
        line(p(12., -11.), p(12., -1.)),
        corner(p(11., -1.), p(12., -1.), p(11., 0.)),
        line(p(11., 0.), p(3., 0.)),
        // the re-entrant corner, turning the other way, about (3, 3)
        arc_to(p(3., 3.), p(3., 0.), p(0., 3.), false, "inside"),
        line(p(0., 3.), p(0., 11.)),
        corner(p(-1., 11.), p(0., 11.), p(-1., 12.)),
        line(p(-1., 12.), p(-11., 12.)),
        corner(p(-11., 11.), p(-11., 12.), p(-12., 11.)),
        line(p(-12., 11.), p(-12., -11.)),
        corner(p(-11., -11.), p(-12., -11.), p(-11., -12.)),
    ]);
    // the pocket from its floor up past the block's top
    let pocket = extrude(&outline, &at([0., 0., 3.]), 6., "pocket").map_err(e)?;
    let body = difference(&body, &pocket).map_err(e)?;
    // one edge of the floor's rim picks the whole tangent chain
    fillet(&body, &[("pocket.start", "pocket.wall")], 1., "round").map_err(e)
}
