use crate::solids::*;
use solidcraft_geom::Seg2;

/// The half section in x and z, the dome's curve one cubic Bézier segment of the profile, turned
/// a full turn about the z axis.
pub fn build() -> R<Body> {
    let (o, rim, top) = (v2(0.0, 0.0), v2(9.0, 0.0), v2(0.0, 10.0));
    let curve = Seg2::Cubic { p0: rim, p1: v2(12.0, 4.0), p2: v2(9.0, 10.0), p3: top };
    let segs = vec![Seg2::Line { a: o, b: rim }, curve, Seg2::Line { a: top, b: o }];
    let outer = Loop2 { segs };
    revolve_z(Region2 { outer, holes: vec![] }, 360.0)
}
