//! A block from (−10, −10, 0) to (10, 10, 10) united with a solid cone on the z axis, its apex at
//! the origin on the block's bottom face and its base the disc of radius 5 at z = 15.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{fuse, primitive_box, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const HALF_WIDTH: f64 = 10.0;
const BLOCK_HEIGHT: f64 = 10.0;
const CONE_HEIGHT: f64 = 15.0;
const CONE_RADIUS: f64 = 5.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    let (block, _) = primitive_box(
        m,
        Point3::new(-HALF_WIDTH, -HALF_WIDTH, 0.0),
        Point3::new(HALF_WIDTH, HALF_WIDTH, BLOCK_HEIGHT),
        &Control::NONE,
    )?;
    // workaround: no cone primitive
    // arris has no cone primitive: the right triangle under its slant in the xz plane, u along x
    // and v along z, its apex at the origin and one leg on the axis, turned a full turn about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    let triangle = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(0.0, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(CONE_RADIUS, CONE_HEIGHT)),
                ProfileSegment::LineTo(p(0.0, CONE_HEIGHT)),
                ProfileSegment::LineTo(p(0.0, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (cone, _) = revolve(m, &triangle, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    let (joined, _) = fuse(m, block, cone, &Control::NONE)?;
    Ok(joined)
}
