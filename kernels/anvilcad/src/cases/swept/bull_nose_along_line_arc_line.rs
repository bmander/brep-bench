//! A block from (−10, −10, −10) to (40, 40, 0) less what an upright bull-nose mill (radius 2, its
//! corner rounded 0.5, its flat end at z = −1.5) passes through, its axis moved from (0, 0) along
//! x to (20, 0), a quarter turn about (20, 10) to (30, 10), and along y to (30, 30).
use std::f64::consts::{FRAC_PI_2, PI};

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::sweep::{PathSegment, sweep};
use anvil_brep::Solid;

use crate::shared::{block, cut_all, e, frame, line, one, region, revolved, v2, v3};

pub fn build() -> Result<Solid, String> {
    // the mill: its half section turned about its axis
    let mill = |x: f64, y: f64| -> Result<Solid, String> {
        let m = revolved(vec![
            line(v2(0., -1.5), v2(1.5, -1.5)),
            Segment::Arc { center: v2(1.5, -1.), radius: 0.5, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
            line(v2(2., -1.), v2(2., 5.)),
            line(v2(2., 5.), v2(0., 5.)),
            line(v2(0., 5.), v2(0., -1.5)),
        ])?;
        Ok(m.transformed(&crate::shared::at([x, y, 0.])))
    };
    // anvil-brep has no sweep of a solid. Upright and moving level along a path that turns no
    // tighter than its radius, the mill sweeps its own section carried square to the path, and
    // itself where it starts and stops
    let path = [
        PathSegment::Line { a: v3(0., 0., 0.), b: v3(20., 0., 0.) },
        PathSegment::Arc { center: v3(20., 10., 0.), axis: v3(0., 0., 1.), start: v3(20., 0., 0.), angle: FRAC_PI_2 },
        PathSegment::Line { a: v3(30., 10., 0.), b: v3(30., 30., 0.) },
    ];
    // the section square to the path at its start, which runs along +x: its sketch x along world y
    // and its sketch y along world z
    let start = frame([0., 0., 0.], [1., 0., 0.], [0., 1., 0.])?;
    let section = Loop2::new(vec![
        line(v2(-2., 5.), v2(-2., -1.)),
        Segment::Arc { center: v2(-1.5, -1.), radius: 0.5, start: PI, sweep: FRAC_PI_2 },
        line(v2(-1.5, -1.5), v2(1.5, -1.5)),
        Segment::Arc { center: v2(1.5, -1.), radius: 0.5, start: -FRAC_PI_2, sweep: FRAC_PI_2 },
        line(v2(2., -1.), v2(2., 5.)),
        line(v2(2., 5.), v2(-2., 5.)),
    ]);
    let groove = one(sweep(&start, &region(&[section])?, &path).map_err(e)?)?;
    cut_all(block([-10., -10., -10.], [40., 40., 0.])?, [Ok(groove), mill(0., 0.), mill(30., 30.)])
}
