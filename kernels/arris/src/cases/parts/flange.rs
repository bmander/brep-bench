//! A round flange on the z axis, a plate of radius 32 and a hub of radius 15 bored through at
//! radius 8, with six bolt holes of radius 3 through the plate on a circle of radius 24.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{cut_many, primitive_cylinder, revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::TAU;

const PLATE_RADIUS: f64 = 32.0;
const PLATE_THICKNESS: f64 = 6.0;
const HUB_RADIUS: f64 = 15.0;
const HUB_TOP: f64 = 16.0;
const BORE_RADIUS: f64 = 8.0;
const BOLT_CIRCLE: f64 = 24.0;
const BOLTS: usize = 6;
const BOLT_RADIUS: f64 = 3.0;
/// How far each drill runs past the plate's faces, so its ends lie clear of them.
const OVERRUN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the half section in the xz plane, u along x and v along z: plate, hub and bore in one
    // outline, turned a full turn about z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    let section = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(BORE_RADIUS, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(PLATE_RADIUS, 0.0)),
                ProfileSegment::LineTo(p(PLATE_RADIUS, PLATE_THICKNESS)),
                ProfileSegment::LineTo(p(HUB_RADIUS, PLATE_THICKNESS)),
                ProfileSegment::LineTo(p(HUB_RADIUS, HUB_TOP)),
                ProfileSegment::LineTo(p(BORE_RADIUS, HUB_TOP)),
                ProfileSegment::LineTo(p(BORE_RADIUS, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (flange, _) = revolve(m, &section, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    // the bolt pattern, one drill every 60° round the bolt circle, all cut at once
    let mut drills = Vec::with_capacity(BOLTS);
    for k in 0..BOLTS {
        let angle = TAU * k as f64 / BOLTS as f64;
        let base = Point3::new(BOLT_CIRCLE * angle.cos(), BOLT_CIRCLE * angle.sin(), -OVERRUN);
        let height = PLATE_THICKNESS + 2.0 * OVERRUN;
        let (drill, _) =
            primitive_cylinder(m, Axis::z_at(base), BOLT_RADIUS, height, &Control::NONE)?;
        drills.push(drill);
    }
    let (drilled, _) = cut_many(m, flange, &drills, &Control::NONE)?;
    Ok(drilled)
}
