//! A block on the square 0 ≤ x, y ≤ 20, its floor at z = 0, under a clamped bicubic B-spline of
//! 6 × 5 control points with a double knot at u = 2/5, its control points at `XS`, `YS` and the
//! heights `HEIGHTS`.
//!
//! Built as `nurbs/bezier-dome` is: the surface an arris `NurbsSurface`, the block's faces
//! assembled round it and finished by `ops::build`.
use super::bezier_dome::block_under;
use crate::Refusal;
use arris::geom::NurbsSurface;
use arris::math::Point3;
use arris::topo::{Body, Model};

/// The knots along u, with the double knot at 2/5, and along v.
const U_KNOTS: [f64; 10] = [0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0];
const V_KNOTS: [f64; 9] = [0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 1.0, 1.0, 1.0, 1.0];
/// xᵢ of every control point of row i, and yⱼ of column j.
const XS: [f64; 6] = [0.0, 3.0, 7.0, 11.0, 16.0, 20.0];
const YS: [f64; 5] = [0.0, 5.0, 10.0, 15.0, 20.0];
const HEIGHTS: [[f64; 5]; 6] = [
    [3.0, 5.0, 6.0, 5.0, 4.0],
    [5.0, 9.0, 11.0, 8.0, 5.0],
    [6.0, 12.0, 15.0, 10.0, 6.0],
    [4.0, 10.0, 18.0, 9.0, 5.0],
    [5.0, 8.0, 9.0, 7.0, 4.0],
    [2.0, 4.0, 5.0, 4.0, 3.0],
];

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let net = (0..6)
        .flat_map(|i| (0..5).map(move |j| Point3::new(XS[i], YS[j], HEIGHTS[i][j])))
        .collect();
    let top = NurbsSurface::new([3, 3], [U_KNOTS.to_vec(), V_KNOTS.to_vec()], net, vec![1.0; 30])
        .map_err(|e| e.to_string())?;
    block_under(m, top)
}
