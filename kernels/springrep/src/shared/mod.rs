//! Helpers shared by the case files: springrep's errors in words, and the frames the cases place
//! their profiles in. Each category's agent-written helpers live in a module of their own here.
#![allow(dead_code)]

use std::f64::consts::TAU;

use springrep::math::{Frame, Vec2, Vec3};
use springrep::profile::NamedSegment;

/// A kernel error as the reason a case gives.
pub fn e(err: springrep::Error) -> String {
    err.to_string()
}

/// The frame at `origin` whose normal is `z` and whose first axis is `x`.
pub fn frame(origin: [f64; 3], z: [f64; 3], x: [f64; 3]) -> Result<Frame, String> {
    Frame::new(
        Vec3::from_array(origin),
        Vec3::from_array(z),
        Vec3::from_array(x),
    )
    .map_err(e)
}

/// The world frame moved to `origin`.
pub fn at(origin: [f64; 3]) -> Frame {
    Frame::new(Vec3::from_array(origin), Vec3::Z, Vec3::X).expect("the world's axes")
}
/// The arc about `center` from `from` to `to`, anticlockwise if `ccw` and else clockwise, of radius
/// the distance from `center` to `from`: for an outline whose pieces are given by their ends.
pub fn arc_to(center: Vec2, from: Vec2, to: Vec2, ccw: bool, name: &str) -> NamedSegment {
    let angle = |p: Vec2| (p.y - center.y).atan2(p.x - center.x);
    let start = angle(from);
    let mut sweep = (angle(to) - start).rem_euclid(TAU);
    if !ccw {
        sweep -= TAU;
    }
    NamedSegment::arc(center, (from - center).norm(), start, sweep, name)
}

pub mod boolean;
pub mod chamfer;
pub mod cnc;
pub mod draft;
pub mod edit;
pub mod feature;
pub mod fill;
pub mod fillet;
pub mod geometry;
pub mod integration;
pub mod loft;
pub mod nurbs;
pub mod offset;
pub mod parts;
pub mod primitives;
pub mod shell;
pub mod singular;
pub mod spline;
pub mod sweep;
pub mod swept;
pub mod tangent;
pub mod thicken;
