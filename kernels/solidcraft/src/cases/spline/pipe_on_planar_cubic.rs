use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_kernel::PathSeg;

/// A sweep path is lines and arcs: a free-form path curve is followed as its polyline, the cubic
/// sampled to 0.01 of its chords, which is how SolidCraft's own sweep feature follows a sketch's
/// Bézier (`path_segments` in its document crate). The kernel then sweeps the profile along the
/// polyline with smooth B-spline walls through stations square to it.
pub fn bezier_path(plane: &Plane, curve: Seg2) -> Vec<PathSeg> {
    let w = |p: Vec2| plane.to_world(p);
    curve.polyline(1e-2).windows(2).map(|q| PathSeg::Line { a: w(q[0]), b: w(q[1]) }).collect()
}

/// The disc of radius 20 in the plane square to the curve where it leaves the origin, along
/// (3, −4, 0), swept along the cubic.
pub fn build() -> R<Body> {
    let curve = Seg2::Cubic {
        p0: v2(0.0, 0.0),
        p1: v2(12.0, -16.0),
        p2: v2(32.0, -16.0),
        p3: v2(44.0, 0.0),
    };
    let start = Plane { origin: Vec3::ZERO, x: v3(0.8, 0.6, 0.0), y: Vec3::Z };
    solidcraft_kernel::sweep(&start, &disc((0.0, 0.0), 20.0), &bezier_path(&Plane::XY, curve))
}
