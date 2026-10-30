//! A block, the box from (−6, −6, −8) to (36, 26, 0), with a groove cut into its top face by a
//! ball of radius 3 whose centre runs along the cubic Bézier curve in the top face with control
//! points (0, 0, 0), (10, 0, 0), (20, 20, 0) and (30, 20, 0).
//!
//! arris has no sweep of a solid, so the ball's sweep is built from what it is by elementary
//! geometry: every point within 3 of the curve. Where the curve turns no tighter than the ball's
//! radius (its tightest turn is about 7.5), that is the tube the ball's great disc square to the
//! curve fills as it runs, arris's sweep of the disc along the curve, with half a ball on each end
//! bulging out of it: the ball's other half lies inside the tube.
use crate::Refusal;
use arris::geom::{FrameLaw, NurbsCurve, Path, PathSegment, Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut, fuse_many, primitive_box, revolve, sweep, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

const BLOCK_LOW: [f64; 3] = [-6.0, -6.0, -8.0];
const BLOCK_HIGH: [f64; 3] = [36.0, 26.0, 0.0];
const CONTROL: [[f64; 3]; 4] =
    [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [20.0, 20.0, 0.0], [30.0, 20.0, 0.0]];
const BALL_RADIUS: f64 = 3.0;

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
pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let [x0, y0, z0] = BLOCK_LOW;
    let [x1, y1, z1] = BLOCK_HIGH;
    let (block, _) =
        primitive_box(m, Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), &Control::NONE)?;
    let points: Vec<Point3> = CONTROL.iter().map(|&[x, y, z]| Point3::new(x, y, z)).collect();
    // a Bézier curve leaves its first point along its first leg and reaches its last along its
    // last leg
    let (start, end) = (points[0], points[3]);
    let (leaving, arriving) = (points[1] - points[0], points[3] - points[2]);
    let bezier = vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
    let curve = NurbsCurve::new(3, bezier, points, vec![1.0; 4]).map_err(|e| e.to_string())?;
    let path = Path { start, segments: vec![PathSegment::Nurbs(curve)] };
    // the ball's great disc square to the curve where it starts; a disc, so how the
    // rotation-minimising frame turns it about the curve does not matter
    let disc = Profile {
        plane: Frame::new(start, leaving, Vec3::z()).map_err(OpError::from)?,
        outer: ProfileLoop::Circle { center: Point2::origin(), radius: BALL_RADIUS },
        holes: vec![],
    };
    let (tube, _) = sweep(m, &disc, &path, FrameLaw::RotationMinimizing, &Control::NONE)?;
    // the halves of the ball outside the tube where it starts and where it ends
    let back = half_ball(m, start, -leaving, BALL_RADIUS)?;
    let front = half_ball(m, end, arriving, BALL_RADIUS)?;
    // the half balls' flat faces are the tube's end caps, which the fuse removes. Each ball's
    // sphere meets the tube's fitted NURBS wall along the end circle, tangent to it all round,
    // and arris 0.9.1 declines that section ("the section of the NURBS surface and the sphere
    // surface is degenerate: turning points of the section that cannot be told apart"), whether
    // the pieces are fused first or cut from the block one by one, half balls or whole.
    let (swept, _) = fuse_many(m, &[tube, back, front], &Control::NONE)?;
    let (grooved, _) = cut(m, block, swept, &Control::NONE)?;
    Ok(grooved)
}
