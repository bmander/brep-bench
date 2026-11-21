//! An L-shaped bracket 10 thick (y from −5 to 5): a post 20 wide and 120 tall with an arm 20 deep
//! out to x = 100 along its top, the inside corner rounded to radius 8.
use std::f64::consts::{FRAC_PI_2, PI};

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Solid, extrude};

use crate::shared::{e, line, region, v2, xz};

pub fn build() -> Result<Solid, String> {
    // the outline in the xz plane, as (x, z)
    let outline = Loop2::new(vec![
        line(v2(0., 0.), v2(20., 0.)),
        line(v2(20., 0.), v2(20., 92.)),
        Segment::Arc { center: v2(28., 92.), radius: 8., start: PI, sweep: -FRAC_PI_2 },
        line(v2(28., 100.), v2(100., 100.)),
        line(v2(100., 100.), v2(100., 120.)),
        line(v2(100., 120.), v2(0., 120.)),
        line(v2(0., 120.), v2(0., 0.)),
    ]);
    // the xz plane's normal is −y, so y from −5 to 5 is its z from −5 to 5
    extrude::extrude(&xz([0., 0., 0.]), &region(&[outline])?, -5., 5.).map_err(e)
}
