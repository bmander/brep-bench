use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let arc = |c: (f64, f64), radius: f64, start: f64, sweep: f64| Seg2::Arc { center: v2(c.0, c.1), radius, start, sweep };
    // the plan's thirteen pieces anticlockwise from (5, 0); the concave arcs turn clockwise
    let plan = Loop2 {
        segs: vec![
            line((5.0, 0.0), (30.0, 0.0)),
            arc((30.0, -5.0), 5.0, FRAC_PI_2, -FRAC_PI_2), // the dip
            arc((45.0, -5.0), 10.0, PI, PI),               // the lobe
            line((55.0, -5.0), (55.0, 0.0)),               // the rise
            arc((60.0, 0.0), 5.0, PI, -FRAC_PI_2),         // the reverse arc
            line((60.0, 5.0), (75.0, 5.0)),
            arc((75.0, 10.0), 5.0, -FRAC_PI_2, FRAC_PI_2),
            line((80.0, 10.0), (80.0, 35.0)),
            arc((75.0, 35.0), 5.0, 0.0, FRAC_PI_2),
            line((75.0, 40.0), (5.0, 40.0)),
            arc((5.0, 35.0), 5.0, FRAC_PI_2, FRAC_PI_2),
            line((0.0, 35.0), (0.0, 5.0)),
            arc((5.0, 5.0), 5.0, PI, FRAC_PI_2),
        ],
    };
    let frame = extrude(&Plane::XY, Region2 { outer: plan, holes: vec![] }, 0.0, 10.0)?;
    // the top rim, picked on the south side and followed all round
    let rim = solidcraft_kernel::tangent_chain(&frame, &[v3(15.0, 0.0, 10.0)])?;
    solidcraft_kernel::fillet(&frame, &rim, 2.0)
}
