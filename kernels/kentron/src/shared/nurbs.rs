//! What the nurbs implementations share: the blocks under a spline top, made by sweeping the top
//! face straight down past the floor (its sides the upright planes through the patch's edges)
//! and cutting that column off at the floor with a box wider than the block; and a ball bounded by
//! a NURBS sphere.
use crate::bench::{p, v};
use kentron::boolean;
use kentron::geometry::surface::{BSplineSurface, BezierSurface, KnotVector, Surface};
use kentron::make;
use kentron::primitives::Cuboid;
use kentron::sweep;
use crate::shared::geometry::{circle_nurbs_periodic, solid_of};
use crate::Refusal;
use kentron::math::Point;
use kentron::topology::{Model, Shape};

/// The block over the floor z = 0 under the face of the whole of `top`, a graph of a height over
/// the plan square from `(lo, lo)` to `(hi, hi)`.
pub fn block_under(m: &mut Model, top: Surface, lo: f64, hi: f64) -> kentron::Result<Shape> {
    let top = make::face_on_surface(m, top)?;
    block_under_face(m, top, lo, hi)
}

/// The block over the floor z = 0 under `top`, a face on a graph of a height over part of the
/// plan square from `(lo, lo)` to `(hi, hi)`.
pub fn block_under_face(m: &mut Model, top: Shape, lo: f64, hi: f64) -> kentron::Result<Shape> {
    let reach = 2.0 * (hi - lo);
    let column = sweep::extrude(m, top, v(0.0, 0.0, -reach))?.shape;
    let floor_up = Cuboid::from_corners(p(lo - 1.0, lo - 1.0, 0.0), p(hi + 1.0, hi + 1.0, reach)).build(m)?;
    boolean::intersection(m, column, floor_up)
}

/// The block under the bicubic Bézier dome of `nurbs/bezier-dome`: the patch's control points
/// stand over an even 4 × 4 grid of the square 0 ≤ x, y ≤ 20 at these heights (row i at
/// x = 20i/3, column j at y = 20j/3).
pub fn bezier_dome(m: &mut Model) -> kentron::Result<Shape> {
    const HEIGHTS: [[f64; 4]; 4] = [[4.0, 6.0, 7.0, 5.0], [6.0, 14.0, 12.0, 7.0], [5.0, 13.0, 16.0, 8.0], [3.0, 6.0, 8.0, 6.0]];
    let poles = (0..4)
        .map(|i| (0..4).map(|j| p(20.0 * i as f64 / 3.0, 20.0 * j as f64 / 3.0, HEIGHTS[i][j])).collect())
        .collect();
    block_under(m, Surface::Bezier(BezierSurface::new(poles)), 0.0, 20.0)
}

/// The block with the weighted barrel-vaulted top of `nurbs/groin-vault`, on the square
/// −12 ≤ x, y ≤ 12: degree 1 along x, knots (0, 0, 1, 1); degree 2 along y, knots
/// (0, 0, 0, 1, 1, 1); for x = ∓12 the control points (x, −12, 10), (x, 0, 26), (x, 12, 10) with
/// weights 1, 3/5, 1, an arch of radius 15 about the line y = 0, z = 1.
pub fn barrel_vault(m: &mut Model) -> kentron::Result<Shape> {
    let arch = [(-12.0, 10.0), (0.0, 26.0), (12.0, 10.0)];
    let poles = [-12.0, 12.0].iter().map(|&x| arch.iter().map(|&(y, z)| p(x, y, z)).collect()).collect();
    let weights = vec![vec![1.0, 0.6, 1.0]; 2];
    let along_x = KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1);
    let along_y = KnotVector::new(vec![0.0, 1.0], vec![3, 3], 2);
    let top = BSplineSurface::try_new(poles, along_x, along_y)?.with_weights(weights);
    block_under(m, Surface::BSpline(top), -12.0, 12.0)
}

/// The ball of `radius` about `center` bounded by the standard biquadratic NURBS sphere of
/// `geometry/nurbs-sphere` moved there: the unit circle round the axis (nine weighted poles on the
/// knots 0, ¼, ½, ¾, 1) times the semicircle (ρ, z) = (0, −r), (r, −r), (r, 0), (r, r), (0, r)
/// on the knots 0, ½, 1, each pole's weight the product, its poles r above and below the center
/// and its seam the meridian on the +x side. As there, the circle is taken in its periodic form,
/// the same curve, since kentron seams a face only on a periodic surface.
pub fn nurbs_ball(m: &mut Model, center: Point, radius: f64) -> Result<Shape, Refusal> {
    let (circle, w, knots, mults) = circle_nurbs_periodic();
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let r = radius;
    let meridian = [(0.0, -r), (r, -r), (r, 0.0), (r, r), (0.0, r)];
    let meridian_w = [1.0, h, 1.0, h, 1.0];
    let poles = circle
        .iter()
        .map(|&(cx, cy)| meridian.iter().map(|&(rho, z)| center + v(rho * cx, rho * cy, z)).collect())
        .collect();
    let weights = w.iter().map(|&wi| meridian_w.iter().map(|&wj| wi * wj).collect()).collect();
    let net = BSplineSurface::try_new(
        poles,
        KnotVector::periodic(knots, mults, 2),
        KnotVector::new(vec![0.0, 0.5, 1.0], vec![3, 2, 3], 2),
    )?
    .with_weights(weights);
    let face = make::face_on_surface(m, Surface::BSpline(net))?;
    solid_of(m, &[face])
}
