use crate::solids::*;
use solidcraft_geom::bspline::{HPoint, bezier_point};

/// The curve's control points.
const CURVE: [(f64, f64, f64); 4] =
    [(0.0, 0.0, 0.0), (12.0, 16.0, 0.0), (26.0, 8.0, 0.0), (24.0, 0.0, 16.0)];

/// The kernel has no space curve: SolidCraft's 3D sketch curves are polylines of points, and a
/// sweep follows one as such (`sweep_path`). So the cubic is handed over as its points,
/// evaluated by `solidcraft_geom`'s de Casteljau and as close to its chords as a sketch's
/// Bézier path is sampled (0.01; the count from the control polygon's second differences, as
/// `Seg2::polyline` takes it).
fn path() -> Vec<Vec3> {
    let ctrl: Vec<HPoint> = CURVE.iter().map(|&(x, y, z)| HPoint { x, y, z, w: 1.0 }).collect();
    let bow = ctrl.windows(3).map(|w| (w[0] - w[1] * 2.0 + w[2]).xyz().len()).fold(0.0, f64::max);
    let n = (bow * 6.0 / (8.0 * 1e-2)).sqrt().ceil() as usize;
    (0..=n).map(|i| bezier_point(&ctrl, i as f64 / n as f64).xyz()).collect()
}

/// The disc of radius 8 in the plane square to the curve where it leaves the origin, along
/// (3, 4, 0), swept along it.
pub fn build() -> R<Body> {
    let start = Plane { origin: Vec3::ZERO, x: v3(-0.8, 0.6, 0.0), y: Vec3::Z };
    solidcraft_kernel::sweep_path(&start, &disc((0.0, 0.0), 8.0), &path())
}
