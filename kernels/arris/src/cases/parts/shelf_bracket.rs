//! An L-shaped bracket 10 thick: a post 20 wide and 120 tall with an arm 20 deep out to x = 100
//! along its top, the inside corner rounded to radius 8, its outline in the xz plane.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Control, Frame, Point2, Point3, Vec3};
use arris::ops::{extrude, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::FRAC_1_SQRT_2;

const POST_WIDTH: f64 = 20.0;
const HEIGHT: f64 = 120.0;
const ARM_DEPTH: f64 = 20.0;
const REACH: f64 = 100.0;
const ROUND: f64 = 8.0;
const THICKNESS: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane at y = −5, u along x and v along z, extruded through the thickness along +y
    let xz = Frame::new(Point3::new(0.0, -THICKNESS / 2.0, 0.0), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    let arm_under = HEIGHT - ARM_DEPTH;
    // the round's centre, a radius off the post's inside and off the arm's underside
    let (cu, cv) = (POST_WIDTH + ROUND, arm_under - ROUND);
    let outline = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(0.0, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(POST_WIDTH, 0.0)),
                ProfileSegment::LineTo(p(POST_WIDTH, cv)),
                // a quarter circle bulging toward the corner (20, 100)
                ProfileSegment::ArcTo {
                    to: p(cu, arm_under),
                    via: p(cu - ROUND * FRAC_1_SQRT_2, cv + ROUND * FRAC_1_SQRT_2),
                },
                ProfileSegment::LineTo(p(REACH, arm_under)),
                ProfileSegment::LineTo(p(REACH, HEIGHT)),
                ProfileSegment::LineTo(p(0.0, HEIGHT)),
                ProfileSegment::LineTo(p(0.0, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (bracket, _) = extrude(m, &outline, Vec3::y(), THICKNESS, &Control::NONE)?;
    Ok(bracket)
}
