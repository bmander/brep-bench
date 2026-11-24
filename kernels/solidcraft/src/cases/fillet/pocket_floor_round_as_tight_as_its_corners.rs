use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let arc = |c: (f64, f64), start: f64| Seg2::Arc { center: v2(c.0, c.1), radius: 4.0, start, sweep: FRAC_PI_2 };
    // the rectangle from (−25, −15) to (25, 15), its corners rounded to 4
    let outline = Loop2 {
        segs: vec![
            line((-21.0, -15.0), (21.0, -15.0)),
            arc((21.0, -11.0), -FRAC_PI_2),
            line((25.0, -11.0), (25.0, 11.0)),
            arc((21.0, 11.0), 0.0),
            line((21.0, 15.0), (-21.0, 15.0)),
            arc((-21.0, 11.0), FRAC_PI_2),
            line((-25.0, 11.0), (-25.0, -11.0)),
            arc((-21.0, -11.0), PI),
        ],
    };
    let block = block((-40.0, -30.0, 0.0), (40.0, 30.0, 16.0))?;
    // cut from the floor z = 6 out past the top
    let pocket = extrude(&Plane::XY, Region2 { outer: outline, holes: vec![] }, 6.0, 17.0)?;
    let pocketed = cut(&block, &pocket)?;
    // the floor's rim, picked on its south side and followed all round
    let rim = solidcraft_kernel::tangent_chain(&pocketed, &[v3(0.0, -15.0, 6.0)])?;
    solidcraft_kernel::fillet(&pocketed, &rim, 4.0)
}
