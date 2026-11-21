//! A four-armed star handle: the outline of four tip circles (radius 4, 33.1 out on the axes) and
//! four fillet circles (radius 3, 15.9 out on the diagonals) joined by straight sides tangent to
//! both, passing between them; extruded 12 along y with a 2° draft inward.
use std::f64::consts::FRAC_PI_2;

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Solid, taper};
use glam::DVec2;

use crate::shared::{e, region, xz};

pub fn build() -> Result<Solid, String> {
    // the side from the tip at (33.1, 0) to the fillet circle at 45°: the line tangent to both,
    // between them, its normal n with n·(c₂ − c₁) = −(r₁ + r₂): it touches the tip circle at
    // (33.8409, 3.9308) and the fillet circle at (10.6873, 8.2949)
    let (c1, r1) = (DVec2::new(33.1, 0.), 4.);
    let (c2, r2) = (DVec2::splat(15.9 / 2f64.sqrt()), 3.);
    let d = c2 - c1;
    let (u, k) = (d.normalize(), (r1 + r2) / d.length());
    let n = -u * k + u.perp() * (1. - k * k).sqrt();
    let (t1, t2) = (c1 - n * r1, c2 + n * r2);
    // the outline anticlockwise, an arm at a time: the tip's arc, a side, the fillet's arc
    // (clockwise, concave), the next arm's side, each arm the first turned a quarter turn
    let rot = |p: DVec2, k: i32| DVec2::from_angle(FRAC_PI_2 * k as f64).rotate(p);
    let mirror = |p: DVec2| DVec2::new(p.y, p.x);
    let angle = |p: DVec2, c: DVec2| (p - c).to_angle();
    let mut segments = Vec::new();
    for k in 0..4 {
        let (tc, fc) = (rot(c1, k), rot(c2, k));
        let (a, b) = (rot(DVec2::new(t1.x, -t1.y), k), rot(t1, k));
        let start = angle(a, tc);
        segments.push(Segment::Arc { center: tc, radius: r1, start, sweep: (angle(b, tc) - start).rem_euclid(std::f64::consts::TAU) });
        segments.push(Segment::Line { a: b, b: rot(t2, k) });
        let (f0, f1) = (rot(t2, k), rot(mirror(t2), k));
        let start = angle(f0, fc);
        segments.push(Segment::Arc { center: fc, radius: r2, start, sweep: -(start - angle(f1, fc)).rem_euclid(std::f64::consts::TAU) });
        segments.push(Segment::Line { a: f1, b: rot(mirror(t1), k) });
    }
    let outline = Loop2::new(segments);
    // in the xz plane, sketch x along x and sketch y along z, its normal −y: y from 0 to 12 is its
    // z from 0 to −12, and its section there moved 12·tan 2° inward. The taper is per unit of the
    // extrusion's z, which grows toward y = 0, so the section grows by tan 2° a unit toward it
    let k = 2f64.to_radians().tan();
    let section = region(&[outline])?;
    taper::extrude_drafted(&xz([0., 0., 0.]), &section, -12., 0., k).map_err(e)
}
