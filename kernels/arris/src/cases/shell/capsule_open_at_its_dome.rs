//! A cylinder of radius 5 from z = 0 to 10 capped by a hemisphere, shelled inward 1 thick, open at
//! its dome, which is tangent to its side.
use crate::solids::{ball, face_through};
use crate::Refusal;
use arris::math::{Axis, Control, Point3};
use arris::ops::{fuse, primitive_cylinder, shell, ShellSide};
use arris::topo::{Body, Model};

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    let capsule = capsule_end(m)?;
    // a point of the dome off its pole and its seam, where the turned half disc leaves a vertex
    // and an edge
    let opening = face_through(m, capsule, Point3::new(0.0, 3.0, HEIGHT + 4.0))?;
    let (shelled, _) =
        shell(m, capsule, &[opening], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 10.0;
const THICKNESS: f64 = 1.0;

/// The cylinder capped by the ball on its top, fused into one solid.
fn capsule_end(m: &mut Model) -> Result<Body, Refusal> {
    let (side, _) =
        primitive_cylinder(m, Axis::z_at(Point3::origin()), RADIUS, HEIGHT, &Control::NONE)?;
    // arris has no sphere primitive: `ball` turns a half disc about its diameter
    let (dome, _) = ball(m, Point3::new(0.0, 0.0, HEIGHT), RADIUS, &Control::NONE)?;
    let (capsule, _) = fuse(m, side, dome, &Control::NONE)?;
    Ok(capsule)
}
