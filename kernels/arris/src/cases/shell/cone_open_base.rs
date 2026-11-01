//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward 1 thick, open at its base.
use crate::solids::face_through;
use crate::Refusal;
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, shell, ShellSide};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const BASE_RADIUS: f64 = 6.0;
const HEIGHT: f64 = 8.0;
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    // arris has no cone primitive: the right triangle under the cone's side, in the xz plane,
    // turned about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let triangle = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: Point2::new(0.0, 0.0),
            segments: vec![
                ProfileSegment::LineTo(Point2::new(BASE_RADIUS, 0.0)),
                ProfileSegment::LineTo(Point2::new(0.0, HEIGHT)),
                ProfileSegment::LineTo(Point2::new(0.0, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (cone, _) = revolve(m, &triangle, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    let base = face_through(m, cone, Point3::new(1.0, 1.0, 0.0))?;
    let (shelled, _) = shell(m, cone, &[base], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}
