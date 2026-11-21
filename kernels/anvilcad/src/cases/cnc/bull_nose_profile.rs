//! A plate from the origin to (120, 120, −16) with a groove milled round a six-lobed flower of
//! arcs by a bull-nose mill of radius 4, its corner rounded to 1.5, at depths 4, 8 and 10.
use std::f64::consts::{FRAC_PI_2, PI};

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::sweep::{PathSegment, sweep};
use anvil_brep::Solid;
use glam::DVec3;

use crate::shared::{block, e, frame, line, one, region, subtract, v2};

pub fn build() -> Result<Solid, String> {
    // the flower: lobes of radius 12 about points 30 from (60, 60) at 0°, 60°, …, valleys of radius
    // d − 12 about points 40 from it at 30°, 90°, …, each valley tangent to the lobes beside it
    let c = DVec3::new(60., 60., -10.);
    let at = |r: f64, deg: f64| c + DVec3::new(deg.to_radians().cos(), deg.to_radians().sin(), 0.) * r;
    let d = (2500. - 1200. * 3f64.sqrt()).sqrt();
    let touch = |lobe: f64, valley: f64| {
        let (l, v) = (at(30., lobe), at(40., valley));
        l + (v - l) / d * 12.
    };
    let turn = |center: DVec3, from: DVec3, to: DVec3, ccw: bool| {
        let (a, b) = (from - center, to - center);
        let mut angle = a.y.atan2(a.x) - b.y.atan2(b.x);
        if ccw {
            angle = -angle;
        }
        let angle = angle.rem_euclid(2. * PI);
        PathSegment::Arc { center, axis: if ccw { DVec3::Z } else { DVec3::NEG_Z }, start: from, angle }
    };
    // anticlockwise from (102, 60), the outermost point of the lobe at 0°: each lobe anticlockwise,
    // each valley clockwise, the lobe at 0° in two pieces
    let start = at(30., 0.) + DVec3::X * 12.;
    let mut path = vec![turn(at(30., 0.), start, touch(0., 30.), true)];
    for k in 0..6 {
        let (lobe, valley, next) = (60. * k as f64, 60. * k as f64 + 30., 60. * (k + 1) as f64);
        path.push(turn(at(40., valley), touch(lobe, valley), touch(next, valley), false));
        let end = if k == 5 { start } else { touch(next, next + 30.) };
        path.push(turn(at(30., next), touch(next, valley), end, true));
    }
    // anvil-brep has no sweep of a solid. The mill, upright and sliding level round a path that
    // turns no tighter than its radius, sweeps its own section carried square to the path: a
    // rectangle 8 wide with its bottom corners rounded to 1.5, from its tip at the deepest lap,
    // z = −10, up past the plate (the shallower laps' grooves lie within it, as do its plunges)
    // the section's frame at (102, 60, −10), square to the path's start (along +y): its sketch x
    // along world x and its sketch y along world −z
    let square = frame(start.to_array(), [0., 1., 0.], [1., 0., 0.])?;
    let section = Loop2::new(vec![
        line(v2(-4., -11.), v2(-4., -1.5)),
        Segment::Arc { center: v2(-2.5, -1.5), radius: 1.5, start: PI, sweep: -FRAC_PI_2 },
        line(v2(-2.5, 0.), v2(2.5, 0.)),
        Segment::Arc { center: v2(2.5, -1.5), radius: 1.5, start: FRAC_PI_2, sweep: -FRAC_PI_2 },
        line(v2(4., -1.5), v2(4., -11.)),
        line(v2(4., -11.), v2(-4., -11.)),
    ]);
    let groove = one(sweep(&square, &region(&[section])?, &path).map_err(e)?)?;
    subtract(&block([0., 0., -16.], [120., 120., 0.])?, &groove)
}
