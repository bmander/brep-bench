//! The half with y ≥ 0 of the torus about the z axis whose tube has radius 2 about a middle circle
//! of radius 6, shelled inward 0.5 thick, open at both its flat faces: a curved pipe.
use crate::solids::face_through;
use crate::Refusal;
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, shell, split, ShellSide};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let t = torus(m)?;
    // split by the plane y = 0, keeping the side its normal +y points to
    let xz = Frame::from_z(Point3::origin(), Vec3::y())?;
    let half = split(m, t, &xz, &Control::NONE)?.positive;
    let ends = [
        face_through(m, half, Point3::new(MAJOR, 0.0, 0.0))?,
        face_through(m, half, Point3::new(-MAJOR, 0.0, 0.0))?,
    ];
    let (pipe, _) = shell(m, half, &ends, THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(pipe)
}

const MAJOR: f64 = 6.0;
const MINOR: f64 = 2.0;
const THICKNESS: f64 = 0.5;

/// arris has no torus primitive: the tube's circle, drawn in the xz plane, turned about z.
fn torus(m: &mut Model) -> Result<Body, Refusal> {
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let tube = Profile {
        plane: xz,
        outer: ProfileLoop::Circle { center: Point2::new(MAJOR, 0.0), radius: MINOR },
        holes: vec![],
    };
    let (torus, _) = revolve(m, &tube, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    Ok(torus)
}
