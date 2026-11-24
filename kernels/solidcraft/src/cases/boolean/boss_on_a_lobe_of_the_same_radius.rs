//! A dog-bone plate, its outline in z = 0 extruded to z = 10: lobes of radius 10 about (±30, 0),
//! a waist along y = ±4, each side running into each lobe through a concave blend of radius 6.
//! The boss, the cylinder of radius 10 about the vertical line through (30, 0) from z = 0 to
//! z = 30, is joined to it: along the lobe's outer arc their sides are one cylinder.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> R<Body> {
    // A blend's centre is 6 off its side (y = ±10) and 16 from its lobe's centre, at x = ±bx; it
    // meets the lobe on the line between the centres, at the angle a above the x axis.
    let a = (10.0f64 / 16.0).asin();
    let bx = 30.0 - 2.0 * 39f64.sqrt();
    let line = |x0: f64, x1: f64, y: f64| Seg2::Line { a: v2(x0, y), b: v2(x1, y) };
    let arc = |cx: f64, cy: f64, radius: f64, start: f64, sweep: f64| Seg2::Arc { center: v2(cx, cy), radius, start, sweep };
    // walked counter-clockwise from the lower side; the concave blends turn clockwise
    let outline = vec![
        line(-bx, bx, -4.0),
        arc(bx, -10.0, 6.0, FRAC_PI_2, a - FRAC_PI_2),
        arc(30.0, 0.0, 10.0, a - PI, 2.0 * (PI - a)),
        arc(bx, 10.0, 6.0, -a, a - FRAC_PI_2),
        line(bx, -bx, 4.0),
        arc(-bx, 10.0, 6.0, -FRAC_PI_2, a - FRAC_PI_2),
        arc(-30.0, 0.0, 10.0, a, 2.0 * (PI - a)),
        arc(-bx, -10.0, 6.0, PI - a, a - FRAC_PI_2),
    ];
    let plate = extrude(&Plane::XY, Region2 { outer: Loop2 { segs: outline }, holes: vec![] }, 0.0, 10.0)?;
    union(&plate, &zcyl(30.0, 0.0, 10.0, 0.0, 30.0)?)
}
