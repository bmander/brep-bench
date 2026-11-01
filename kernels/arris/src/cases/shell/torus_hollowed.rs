//! A torus about the z axis whose tube has radius 2 about a middle circle of radius 6, hollowed to
//! a wall 0.5 thick with no opening.
use crate::Refusal;
use arris::geom::{Profile, ProfileLoop};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, shell, ShellSide};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let t = torus(m)?;
    // no openings: the cavity is a closed void, a second shell inside the first
    let (hollow, _) = shell(m, t, &[], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(hollow)
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
