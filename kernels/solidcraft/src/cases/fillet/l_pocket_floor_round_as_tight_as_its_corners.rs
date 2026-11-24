use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let arc = |c: (f64, f64), radius: f64, start: f64, sweep: f64| Seg2::Arc { center: v2(c.0, c.1), radius, start, sweep };
    // the L, anticlockwise, its convex corners rounded to 1 and its re-entrant one to 3 (clockwise)
    let ell = Loop2 {
        segs: vec![
            line((-11.0, -12.0), (11.0, -12.0)),
            arc((11.0, -11.0), 1.0, -FRAC_PI_2, FRAC_PI_2),
            line((12.0, -11.0), (12.0, -1.0)),
            arc((11.0, -1.0), 1.0, 0.0, FRAC_PI_2),
            line((11.0, 0.0), (3.0, 0.0)),
            arc((3.0, 3.0), 3.0, -FRAC_PI_2, -FRAC_PI_2),
            line((0.0, 3.0), (0.0, 11.0)),
            arc((-1.0, 11.0), 1.0, 0.0, FRAC_PI_2),
            line((-1.0, 12.0), (-11.0, 12.0)),
            arc((-11.0, 11.0), 1.0, FRAC_PI_2, FRAC_PI_2),
            line((-12.0, 11.0), (-12.0, -11.0)),
            arc((-11.0, -11.0), 1.0, PI, FRAC_PI_2),
        ],
    };
    let block = block((-20.0, -20.0, 0.0), (20.0, 20.0, 8.0))?;
    // cut from the floor z = 3 out past the top
    let pocket = extrude(&Plane::XY, Region2 { outer: ell, holes: vec![] }, 3.0, 9.0)?;
    let pocketed = cut(&block, &pocket)?;
    // the floor's rim, picked on its south side and followed all round
    let rim = solidcraft_kernel::tangent_chain(&pocketed, &[v3(0.0, -12.0, 3.0)])?;
    solidcraft_kernel::fillet(&pocketed, &rim, 1.0)
}
