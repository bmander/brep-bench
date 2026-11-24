//! A plate from z = 0 to 12 whose outline is six lobes, arcs of the circles of radius 8 about the
//! points 40 from the z axis at 0°, 60°, …, and six hollows, arcs of the circles of radius 30 that
//! touch each two lobes beside them from outside: twelve arcs in one tangent loop. Both its rims
//! are rounded to radius 1, each as one round.
use std::f64::consts::PI;

use springrep::blend::fillet;
use springrep::math::Vec2;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{arc_to, at, e};

pub fn build() -> Result<Brep, String> {
    let polar = |r: f64, a: f64| Vec2::new(r * a.cos(), r * a.sin());
    // each hollow's centre is 38 from the two lobes' centres beside it, on the ray between them,
    // the farther of the two such points from the axis
    let d = 20. * 3f64.sqrt() + 6. * 29f64.sqrt();
    let lobe = |k: usize| polar(40., k as f64 * PI / 3.);
    let hollow = |k: usize| polar(d, (2 * k + 1) as f64 * PI / 6.);
    // where the lobe about `l` touches the hollow about `h`: 8 along the line between
    let touch = |l: Vec2, h: Vec2| l + (h - l) * (8. / 38.);
    let mut outline = Vec::new();
    for k in 0..6 {
        let (l, before, after) = (lobe(k), hollow((k + 5) % 6), hollow(k));
        outline.push(arc_to(l, touch(l, before), touch(l, after), true, "lobe"));
        let next = lobe((k + 1) % 6);
        outline.push(arc_to(after, touch(l, after), touch(next, after), false, "hollow"));
    }
    let plate = extrude(&Profile::outline(outline), &at([0., 0., 0.]), 12., "plate").map_err(e)?;
    // one edge of each rim picks its whole tangent chain
    let rims = [("plate.end", "plate.lobe"), ("plate.start", "plate.lobe")];
    fillet(&plate, &rims, 1., "round").map_err(e)
}
