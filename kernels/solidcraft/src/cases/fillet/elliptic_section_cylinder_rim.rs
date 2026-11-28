//! No layer has a round with an elliptic section, or one sized by a distance on each face (the
//! kernel's and ops's rounds have a circular, curvature-continuous or chord-sized section of one
//! size, and their chamfers, even with a distance on each face, a flat one; solidcore's
//! two-radius solvers make circular arcs), so the cylinder is cut by the
//! corner outside the quarter ellipse instead: the quarter ellipse from (4, 10) to (5, 7) about
//! the corner (5, 10) in ρ and z (a conic of weight √2/2), closed round the corner outside the
//! cylinder and turned round the z axis.
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> R<Body> {
    let c = zcyl(0.0, 0.0, 5.0, 0.0, 10.0)?;
    let (on_top, on_side) = (v2(4.0, 10.0), v2(5.0, 7.0));
    let mut segs = vec![Seg2::Conic { a: on_top, apex: v2(5.0, 10.0), b: on_side, w: FRAC_1_SQRT_2 }];
    let around = [on_side, v2(6.0, 7.0), v2(6.0, 11.0), v2(4.0, 11.0), on_top];
    segs.extend(around.windows(2).map(|p| Seg2::Line { a: p[0], b: p[1] }));
    let outside = Region2 { outer: Loop2 { segs }, holes: vec![] };
    cut(&c, &revolve_z(outside, 360.0)?)
}
