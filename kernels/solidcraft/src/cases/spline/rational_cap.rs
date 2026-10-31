use crate::solids::*;
use solidcraft_geom::Seg2;

/// The half section in x and z, its curve the profile's conic: a rational quadratic Bézier
/// segment whose apex has the weight ½ (the ends weigh 1), turned a full turn about the z axis.
pub fn build() -> R<Body> {
    let s = 5.0 * 3f64.sqrt();
    let (foot, rim, top) = (v2(0.0, -5.0), v2(s, -5.0), v2(0.0, 10.0));
    let arc = Seg2::Conic { a: rim, apex: v2(2.0 * s, 10.0), b: top, w: 0.5 };
    let segs = vec![Seg2::Line { a: foot, b: rim }, arc, Seg2::Line { a: top, b: foot }];
    let outer = Loop2 { segs };
    revolve_z(Region2 { outer, holes: vec![] }, 360.0)
}
