//! A block from z = 0 to 10 over a plan 80 by 40 with its corners rounded to radius 5 and a lobe
//! of radius 10 hanging from its south side, which climbs out of the lobe by a straight rise and a
//! reverse arc onto a step: thirteen pieces in one tangent chain. The rim of its top is rounded to
//! radius 2 as one round.
use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::{NamedSegment, Profile};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{arc_to, at, e};

pub fn build() -> Result<Brep, String> {
    let p = |x: f64, y: f64| Vec2::new(x, y);
    let line = NamedSegment::line;
    // anticlockwise from the south-west, as the case's table lists them
    let plan = Profile::outline(vec![
        line(p(5., 0.), p(30., 0.), "south"),
        arc_to(p(30., -5.), p(30., 0.), p(35., -5.), false, "dip"),
        arc_to(p(45., -5.), p(35., -5.), p(55., -5.), true, "lobe"),
        line(p(55., -5.), p(55., 0.), "rise"),
        arc_to(p(60., 0.), p(55., 0.), p(60., 5.), false, "reverse"),
        line(p(60., 5.), p(75., 5.), "step"),
        arc_to(p(75., 10.), p(75., 5.), p(80., 10.), true, "corner"),
        line(p(80., 10.), p(80., 35.), "east"),
        arc_to(p(75., 35.), p(80., 35.), p(75., 40.), true, "corner"),
        line(p(75., 40.), p(5., 40.), "north"),
        arc_to(p(5., 35.), p(5., 40.), p(0., 35.), true, "corner"),
        line(p(0., 35.), p(0., 5.), "west"),
        arc_to(p(5., 5.), p(0., 5.), p(5., 0.), true, "corner"),
    ]);
    let block = extrude(&plan, &at([0., 0., 0.]), 10., "block").map_err(e)?;
    // one edge of the rim picks the whole tangent chain
    fillet(&block, &[("block.end", "block.north")], 2., "round").map_err(e)
}
