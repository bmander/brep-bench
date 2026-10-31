use super::bezier_dome::{block_under, sheet};
use crate::solids::*;
use solidcraft_geom::Seg2;
use solidcraft_geom::bspline::{BezierPatch, HPoint, NurbsSurface};
use solidcraft_geom::tspline::TSpline;

/// The knots along u, a double knot at 2/5.
const U: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0];
/// The knots along v.
const V: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0];
/// The control points' x (row i) and y (column j).
const X: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const Y: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
/// The heights zᵢⱼ.
const Z: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

/// A Bézier patch of the surface as a bicubic tensor surface of its own.
fn tensor(p: &BezierPatch) -> Result<NurbsSurface, String> {
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    NurbsSurface::from_homogeneous([3, 3], [bezier.clone(), bezier], p.ctrl.clone())
        .map_err(|e| e.to_string())
}

/// The block under the B-spline surface.
///
/// A sketch region takes Béziers but no B-spline, so each side's top edge goes to the kernel as
/// the Bézier pieces of the surface's boundary curve there, split at its inner knot. The kernel
/// stitches faces edge for edge, so the surface goes as its Bézier patches as well (the faces
/// `tspline_sheet` makes of a T-spline with T-junctions), one sheet each, to meet the sides'
/// pieces. Both come from `solidcraft_geom`'s Bézier extraction of the T-spline the surface is
/// (`TSpline::bezier_elements`), so they are the surface exactly, and share their points.
pub fn build() -> Result<Body, crate::Refusal> {
    let pts: Vec<Vec3> =
        (0..5).flat_map(|j| (0..6).map(move |i| v3(X[i], Y[j], Z[i][j]))).collect();
    let surface = NurbsSurface::new([3, 3], [U.to_vec(), V.to_vec()], &pts, &[1.0; 30])
        .map_err(|e| e.to_string())?;
    let mut patches = TSpline::from_nurbs(&surface)
        .and_then(|t| t.bezier_elements())
        .map_err(|e| e.to_string())?;
    patches.sort_by(|a, b| a.t[0].total_cmp(&b.t[0]).then(a.s[0].total_cmp(&b.s[0])));
    let sheets: Vec<Body> =
        patches.iter().map(|p| sheet(&tensor(p)?)).collect::<Result<_, crate::Refusal>>()?;
    // The patches' sides on the boundary, in order along it: side 0 of the patches at v = 0, 1
    // at u = 1, 2 at v = 1 and 3 at u = 0 (see `BezierPatch::side`), in their side's own x
    // (world x along y = 0 and y = 20, world y along x = 0 and x = 20) and z.
    let along = |on: &dyn Fn(&BezierPatch) -> bool, side: usize, x: fn(&HPoint) -> f64| {
        patches
            .iter()
            .filter(|p| on(p))
            .map(|p| {
                let c: Vec<Vec2> = p.side(side).iter().map(|h| v2(x(h) / h.w, h.z / h.w)).collect();
                Seg2::Cubic { p0: c[0], p1: c[1], p2: c[2], p3: c[3] }
            })
            .collect::<Vec<_>>()
    };
    let edges = [
        along(&|p| p.t[0] == 0.0, 0, |h| h.x),
        along(&|p| p.s[1] == 1.0, 1, |h| h.y),
        along(&|p| p.t[1] == 1.0, 2, |h| h.x),
        along(&|p| p.s[0] == 0.0, 3, |h| h.y),
    ];
    block_under(&sheets, edges)
}
