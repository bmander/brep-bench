use crate::solids::*;
use solidcraft_geom::Seg2;
use std::f64::consts::PI;

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let section = Loop2 {
        segs: vec![
            line((0.0, 0.0), (20.0, 0.0)),
            line((20.0, 0.0), (20.0, 60.0)),
            line((20.0, 60.0), (16.0, 60.0)),
            // a quarter circle about (16, 64), clockwise from (16, 60) to (12, 64)
            Seg2::Arc { center: v2(16.0, 64.0), radius: 4.0, start: -PI / 2.0, sweep: -PI / 2.0 },
            line((12.0, 64.0), (12.0, 160.0)),
            line((12.0, 160.0), (0.0, 160.0)),
            line((0.0, 160.0), (0.0, 0.0)),
        ],
    };
    revolve_z(Region2 { outer: section, holes: vec![] }, 360.0)
}
