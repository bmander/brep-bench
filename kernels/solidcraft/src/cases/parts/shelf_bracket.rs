use crate::solids::*;
use solidcraft_geom::Seg2;
use std::f64::consts::PI;

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let outline = Loop2 {
        segs: vec![
            line((0.0, 0.0), (20.0, 0.0)),
            line((20.0, 0.0), (20.0, 92.0)),
            // a quarter circle about (28, 92), clockwise from (20, 92) to (28, 100)
            Seg2::Arc { center: v2(28.0, 92.0), radius: 8.0, start: PI, sweep: -PI / 2.0 },
            line((28.0, 100.0), (100.0, 100.0)),
            line((100.0, 100.0), (100.0, 120.0)),
            line((100.0, 120.0), (0.0, 120.0)),
            line((0.0, 120.0), (0.0, 0.0)),
        ],
    };
    extrude(&xz(), Region2 { outer: outline, holes: vec![] }, -5.0, 5.0)
}
