//! A groin vault: the common part of a block on the square −12 ≤ x, y ≤ 12 under a weighted
//! barrel vault arching along y, and the same block turned a quarter turn about z.
//!
//! Each block is built as `nurbs/bezier-dome` is: the vault, a rational B-spline surface, an
//! arris `NurbsSurface`, the block's faces assembled round it and finished by `ops::build`.
use super::bezier_dome::block_under;
use crate::Refusal;
use arris::geom::NurbsSurface;
use arris::math::nalgebra::UnitQuaternion;
use arris::math::{Control, Isometry, Point3, Vec3};
use arris::ops::{common, transform};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_PI_2;

const HALF: f64 = 12.0;
/// (y, z) and the weight of each control point along y, the same for x = −12 and for x = 12.
const ARCH: [(f64, f64, f64); 3] = [(-12.0, 10.0, 1.0), (0.0, 26.0, 0.6), (12.0, 10.0, 1.0)];

/// The block under the vault arching along y: degree 1 along x on the knots (0, 0, 1, 1) and 2
/// along y on (0, 0, 0, 1, 1, 1), each row a rational quadratic Bézier arc.
pub fn vault(m: &mut Model) -> Result<Body, Refusal> {
    let net = [-HALF, HALF]
        .iter()
        .flat_map(|&x| ARCH.iter().map(move |&(y, z, _)| Point3::new(x, y, z)))
        .collect();
    let weights = [-HALF, HALF].iter().flat_map(|_| ARCH.map(|(_, _, w)| w)).collect();
    let knots = [vec![0.0, 0.0, 1.0, 1.0], vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];
    let top = NurbsSurface::new([1, 2], knots, net, weights).map_err(|e| e.to_string())?;
    block_under(m, top)
}

// workaround: no solid from a NURBS surface
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let first = vault(m)?;
    let turn = UnitQuaternion::from_axis_angle(&Vec3::z_axis(), FRAC_PI_2);
    let quarter = Isometry::from_rotation(turn);
    let (second, _) = transform(m, first, &quarter, &Control::NONE)?;
    let (groin, _) = common(m, first, second, &Control::NONE)?;
    Ok(groin)
}
