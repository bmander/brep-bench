//! The ball of radius 5 centred on the origin below the plane z = 3.9999, shelled inward 1 thick,
//! open at its flat face, which the cavity barely reaches.
use crate::solids::{ball, face_through};
use crate::Refusal;
use arris::math::{Control, Frame, Point3, Vec3};
use arris::ops::{shell, split, ShellSide};
use arris::topo::{Body, Model};

const RADIUS: f64 = 5.0;
const HEIGHT: f64 = 3.9999;
const THICKNESS: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, Refusal> {
    // arris has no sphere primitive: `ball` turns a half disc about its diameter
    let (sphere, _) = ball(m, Point3::origin(), RADIUS, &Control::NONE)?;
    // split by the plane z = HEIGHT, keeping the side its normal −z points to
    let cut = Frame::from_z(Point3::new(0.0, 0.0, HEIGHT), -Vec3::z())?;
    let below = split(m, sphere, &cut, &Control::NONE)?.positive;
    let flat = face_through(m, below, Point3::new(0.0, 0.0, HEIGHT))?;
    let (shelled, _) = shell(m, below, &[flat], THICKNESS, ShellSide::Inward, &Control::NONE)?;
    Ok(shelled)
}
