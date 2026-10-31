use crate::solids::*;
use solidcraft_geom::Seg2;

/// A profile takes no rational B-spline, only its pieces: the double knot at ½ makes the curve
/// two rational quadratic Bézier segments, each a conic of the profile through three of the
/// control points with the middle one's weight √2/2 (the ends weigh 1). The half section in x
/// and z, turned a full turn about the z axis.
pub fn build() -> R<Body> {
    let w = 0.5f64.sqrt();
    let (bottom, waist, top) = (v2(0.0, -10.0), v2(6.0, 0.0), v2(0.0, 10.0));
    let lower = Seg2::Conic { a: bottom, apex: v2(6.0, -10.0), b: waist, w };
    let upper = Seg2::Conic { a: waist, apex: v2(6.0, 10.0), b: top, w };
    let outer = Loop2 { segs: vec![lower, upper, Seg2::Line { a: top, b: bottom }] };
    revolve_z(Region2 { outer, holes: vec![] }, 360.0)
}
