//! The half of the cylinder of radius 5 about the z axis from z = 0 to z = 10 where y ≥ 0, every
//! edge rounded to radius 1.
use std::f64::consts::PI;

use anvil_brep::profile::{Loop2, Segment};
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{e, fillet, line, region, v2};

pub fn build() -> Result<Solid, String> {
    let half = Loop2::new(vec![
        Segment::Arc { center: v2(0., 0.), radius: 5., start: 0., sweep: PI },
        line(v2(-5., 0.), v2(5., 0.)),
    ]);
    let bar = extrude::extrude(&Frame::world(), &region(&[half])?, 0., 10.).map_err(e)?;
    // the two rims, the two straight edges of the round face and the flat face's two ends
    let edges = [[0., 5., 0.], [0., 5., 10.], [5., 0., 5.], [-5., 0., 5.], [0., 0., 0.], [0., 0., 10.]];
    fillet(&bar, &edges, 1.)
}
