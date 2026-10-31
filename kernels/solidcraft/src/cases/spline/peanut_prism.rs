use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_geom::bspline::{HPoint, insert_knot};

/// The peanut's twelve control points, counter-clockwise.
const PEANUT: [(f64, f64); 12] = [
    (14.0, 0.0),
    (12.0, 8.0),
    (4.0, 6.0),
    (0.0, -21.0 / 8.0),
    (-4.0, 6.0),
    (-12.0, 8.0),
    (-14.0, 0.0),
    (-12.0, -8.0),
    (-4.0, -6.0),
    (0.0, 21.0 / 8.0),
    (4.0, -6.0),
    (12.0, -8.0),
];

/// The Bézier pieces of the closed uniform cubic B-spline over `ctrl`, wrapping round. A
/// profile takes lines, arcs and Béziers but no B-spline, so the curve is handed over as its
/// pieces, as SolidCraft's sketches hand over a control-point spline (`bspline_beziers`): here
/// by `solidcraft_geom`'s knot insertion. The periodic curve is the open one over the points
/// with the first three repeated at the end, on the knots 0, 1, …, n + 6, over [3, n + 3];
/// every whole knot in that domain inserted to multiplicity 3 leaves the pieces' control points
/// in order, the points whose Greville abscissae lie in it.
pub fn periodic_cubic(ctrl: &[(f64, f64)]) -> Result<Vec<Seg2>, String> {
    let n = ctrl.len();
    let mut pts: Vec<HPoint> =
        (0..n + 3).map(|i| ctrl[i % n]).map(|(x, y)| HPoint { x, y, z: 0.0, w: 1.0 }).collect();
    let mut knots: Vec<f64> = (0..n + 7).map(|k| k as f64).collect();
    for u in 3..=n + 3 {
        let u = u as f64;
        while knots.iter().filter(|&&k| k == u).count() < 3 {
            (knots, pts) = insert_knot(3, &knots, &pts, u).map_err(|e| e.to_string())?;
        }
    }
    let (lo, hi) = (3.0, (n + 3) as f64);
    let greville = |i: usize| (knots[i + 1] + knots[i + 2] + knots[i + 3]) / 3.0;
    let inside: Vec<Vec2> = (0..pts.len())
        .filter(|&i| (lo..=hi).contains(&greville(i)))
        .map(|i| v2(pts[i].x, pts[i].y))
        .collect();
    Ok(inside
        .windows(4)
        .step_by(3)
        .map(|c| Seg2::Cubic { p0: c[0], p1: c[1], p2: c[2], p3: c[3] })
        .collect())
}

/// The peanut on the xy plane, extruded 10 up.
pub fn build() -> Result<Body, crate::Refusal> {
    let outer = Loop2 { segs: periodic_cubic(&PEANUT)? };
    Ok(extrude(&Plane::XY, Region2 { outer, holes: vec![] }, 0.0, 10.0)?)
}
