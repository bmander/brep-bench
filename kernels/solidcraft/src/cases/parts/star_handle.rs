use crate::solids::*;
use solidcraft_geom::Seg2;
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, TAU};

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;

/// The point turned `k` quarter turns about the origin.
fn quarter(p: Vec2, k: usize) -> Vec2 {
    match k % 4 {
        0 => p,
        1 => v2(-p.y, p.x),
        2 => v2(-p.x, -p.y),
        _ => v2(p.y, -p.x),
    }
}

/// The star's outline, as a sketch of it would be drawn: tip arcs, fillet arcs and the sides
/// tangent to both, in x and z.
fn outline() -> Loop2 {
    let d = FILLET_DISTANCE * FRAC_1_SQRT_2;
    let (tip, fillet) = (v2(TIP_DISTANCE, 0.0), v2(d, d));
    // The first arm's side toward the diagonal, the line n·p = c passing between the circles:
    // the tip circle a radius on its positive side and the fillet circle a radius on its
    // negative, so n·(tip − fillet) is the sum of the radii. Of the two such lines it is the one
    // touching the tip circle on the diagonal's side of the axis.
    let between = tip - fillet;
    let turn = between.y.atan2(between.x)
        - ((TIP_RADIUS + FILLET_RADIUS) / (between.x.hypot(between.y))).acos();
    let n = v2(turn.cos(), turn.sin());
    let on_tip = tip - n * TIP_RADIUS;
    let on_fillet = fillet + n * FILLET_RADIUS;
    let mirror = |p: Vec2| v2(p.x, -p.y);
    // the tip's outer arc from −a to a, crossing the axis; the fillet's inner arc clockwise from
    // where the side touches it to its mirror image across the diagonal
    let a = (on_tip.y - tip.y).atan2(on_tip.x - tip.x);
    let f = (on_fillet.y - fillet.y).atan2(on_fillet.x - fillet.x);
    let f_sweep = ((FRAC_PI_2 - f) - f).rem_euclid(TAU) - TAU;
    let mut segs = Vec::with_capacity(16);
    for k in 0..4 {
        let turn = k as f64 * FRAC_PI_2;
        let from = quarter(mirror(on_fillet), k);
        segs.push(Seg2::Line { a: from, b: quarter(mirror(on_tip), k) });
        segs.push(Seg2::Arc {
            center: quarter(tip, k),
            radius: TIP_RADIUS,
            start: -a + turn,
            sweep: 2.0 * a,
        });
        segs.push(Seg2::Line { a: quarter(on_tip, k), b: quarter(on_fillet, k) });
        segs.push(Seg2::Arc {
            center: quarter(fillet, k),
            radius: FILLET_RADIUS,
            start: f + turn,
            sweep: f_sweep,
        });
    }
    Loop2 { segs }
}

pub fn build() -> R<Body> {
    let star = Region2 { outer: outline(), holes: vec![] };
    // 12 along +y (the plane's normal is −y), every side drafted 2° inward
    solidcraft_kernel::extrude_tapered(&xz(), &star, 12.0, -1.0, -2f64.to_radians())
}
