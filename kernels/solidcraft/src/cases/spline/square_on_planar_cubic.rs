use super::pipe_on_planar_cubic::bezier_path;
use crate::solids::*;
use solidcraft_geom::Seg2;

/// The square of side 20 in the plane square to the curve where it leaves the origin, along
/// (4, 3, 0), its sketch x horizontal and its sketch y up, swept along the cubic's polyline (as
/// `spline/pipe-on-planar-cubic` is). The stations' frames turn least about a planar path, so
/// the square's top and bottom stay level all the way.
pub fn build() -> R<Body> {
    let curve = Seg2::Cubic {
        p0: v2(0.0, 0.0),
        p1: v2(16.0, 12.0),
        p2: v2(18.0, 26.0),
        p3: v2(12.0, 34.0),
    };
    let start = Plane { origin: Vec3::ZERO, x: v3(-0.6, 0.8, 0.0), y: Vec3::Z };
    let square = rect(-10.0, -10.0, 10.0, 10.0);
    solidcraft_kernel::sweep(&start, &square, &bezier_path(&Plane::XY, curve))
}
