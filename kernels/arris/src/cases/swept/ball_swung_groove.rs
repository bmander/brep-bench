//! A block, the box from (−6, −6, −8) to (18, 18, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre swings a quarter turn about the z axis in the top face, from
//! (12, 0, 0) to (0, 12, 0).
//!
//! arris has no sweep of a solid, so the ball's sweep is built from what it is by elementary
//! geometry: every point within 3 of the quarter circle its centre runs along. That is the tube
//! the ball's great disc square to the circle fills as it turns, a quarter of a torus, with half a
//! ball on each end, bulging out of the turn: the ball's other half lies inside the tube.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, fuse_many, primitive_box, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, TAU};

const BLOCK_LOW: [f64; 3] = [-6.0, -6.0, -8.0];
const BLOCK_HIGH: [f64; 3] = [18.0, 18.0, 0.0];
const BALL_RADIUS: f64 = 3.0;
/// The centre's distance from the z axis, which it keeps as it swings.
const SWING_RADIUS: f64 = 12.0;
const SWING: f64 = FRAC_PI_2;

/// A half ball of `radius` on the disc about `centre` square to `outward`, bulging along it: the
/// quarter disc in a plane through `outward`, turned about it.
fn half_ball(m: &mut Model, centre: Point3, outward: Vec3, radius: f64) -> Result<Body, OpError> {
    let across = Frame::from_z(centre, outward)?.x().into_inner();
    let (r, h) = (radius, radius * FRAC_1_SQRT_2);
    let quarter_disc = Profile {
        plane: Frame::new(centre, across.cross(&outward), across)?,
        outer: ProfileLoop::Path {
            start: Point2::origin(),
            segments: vec![
                ProfileSegment::LineTo(Point2::new(r, 0.0)),
                ProfileSegment::ArcTo { to: Point2::new(0.0, r), via: Point2::new(h, h) },
                ProfileSegment::LineTo(Point2::origin()),
            ],
        },
        holes: vec![],
    };
    Ok(revolve(m, &quarter_disc, Axis::new(centre, outward)?, TAU, &Control::NONE)?.0)
}

// workaround: no sweep of a solid, no sphere primitive
pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let [x0, y0, z0] = BLOCK_LOW;
    let [x1, y1, z1] = BLOCK_HIGH;
    let (block, _) =
        primitive_box(m, Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), &Control::NONE)?;
    // the ball's great disc in the xz plane where it starts, turned with it a quarter turn about
    // z: the xz plane's normal −y, so a positive turn about z leaves it towards +y
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let disc = Profile {
        plane: xz,
        outer: ProfileLoop::Circle { center: Point2::new(SWING_RADIUS, 0.0), radius: BALL_RADIUS },
        holes: vec![],
    };
    let (tube, _) = revolve(m, &disc, Axis::z_at(Point3::origin()), SWING, &Control::NONE)?;
    // the ends: the ball leaves its start going +y and arrives at its end going −x, so the halves
    // outside the tube bulge −y at the start and −x at the end
    let start = Point3::new(SWING_RADIUS, 0.0, 0.0);
    let end = Point3::new(0.0, SWING_RADIUS, 0.0);
    let back = half_ball(m, start, -Vec3::y(), BALL_RADIUS)?;
    let front = half_ball(m, end, -Vec3::x(), BALL_RADIUS)?;
    // the half balls' flat faces are the tube's end caps, which the fuse removes
    let (swept, _) = fuse_many(m, &[tube, back, front], &Control::NONE)?;
    let (grooved, _) = cut(m, block, swept, &Control::NONE)?;
    Ok(grooved)
}
