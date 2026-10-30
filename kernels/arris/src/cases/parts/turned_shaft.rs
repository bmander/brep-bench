//! A shaft turned about the z axis: radius 20 from z = 0 to 60 and radius 12 from 60 to 160,
//! the inside corner of the shoulder rounded to radius 4.
use arris::geom::{Profile, ProfileLoop, ProfileSegment};
use arris::math::{Axis, Control, Frame, Point2, Point3, Vec3};
use arris::ops::{revolve, OpError};
use arris::topo::{Body, Model};
use std::f64::consts::{FRAC_1_SQRT_2, TAU};

const THICK_RADIUS: f64 = 20.0;
const SHOULDER: f64 = 60.0;
const THIN_RADIUS: f64 = 12.0;
const LENGTH: f64 = 160.0;
const ROUND: f64 = 4.0;

pub fn build(m: &mut Model) -> Result<Body, OpError> {
    // the xz plane: u along x, v along z
    let xz = Frame::new(Point3::origin(), -Vec3::y(), Vec3::x())?;
    let p = Point2::new;
    // the round's centre, a radius off the shoulder face and off the thin part's side
    let (cu, cv) = (THIN_RADIUS + ROUND, SHOULDER + ROUND);
    let section = Profile {
        plane: xz,
        outer: ProfileLoop::Path {
            start: p(0.0, 0.0),
            segments: vec![
                ProfileSegment::LineTo(p(THICK_RADIUS, 0.0)),
                ProfileSegment::LineTo(p(THICK_RADIUS, SHOULDER)),
                ProfileSegment::LineTo(p(cu, SHOULDER)),
                // a quarter circle bulging toward the corner (12, 60)
                ProfileSegment::ArcTo {
                    to: p(THIN_RADIUS, cv),
                    via: p(cu - ROUND * FRAC_1_SQRT_2, cv - ROUND * FRAC_1_SQRT_2),
                },
                ProfileSegment::LineTo(p(THIN_RADIUS, LENGTH)),
                ProfileSegment::LineTo(p(0.0, LENGTH)),
                ProfileSegment::LineTo(p(0.0, 0.0)),
            ],
        },
        holes: vec![],
    };
    let (shaft, _) = revolve(m, &section, Axis::z_at(Point3::origin()), TAU, &Control::NONE)?;
    Ok(shaft)
}
