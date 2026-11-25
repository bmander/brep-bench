//! A dog-bone plate (two lobes of radius 10 about (±30, 0) joined by a waist between y = ±4, each
//! side running into each lobe through a concave blend of radius 6) from z = 0 to 10, united with
//! a boss of the lobe's own radius on the same axis (the cylinder of radius 10 about the upright
//! through (30, 0), from z = 0 to 30).
//!
//! The plate is its eight-piece outline extruded, the boss a circle extruded, and the two
//! united: the boss's side and the lobe's outer arc are one cylinder.
use crate::bench::{arc, cylinder, extrude, p2, p3, union, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};
use std::f64::consts::PI;

/// The arc of radius `r` about `c` from `a` to `b`, anticlockwise or clockwise.
fn arc_between(c: (f64, f64), r: f64, a: (f64, f64), b: (f64, f64), anticlockwise: bool) -> Curve2 {
    let (sa, sb) = ((a.1 - c.1).atan2(a.0 - c.0), (b.1 - c.1).atan2(b.0 - c.0));
    let sweep = if anticlockwise { (sb - sa).rem_euclid(2.0 * PI) } else { -(sa - sb).rem_euclid(2.0 * PI) };
    arc(p2(c.0, c.1), r, sa.to_degrees(), sweep.to_degrees())
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let xc = 30.0 - 2.0 * 39f64.sqrt();
    // Where the blend about (±xc, ±10) touches the lobe about (±30, 0): 10 along the line
    // between their centres.
    let touch = |sx: f64, sy: f64| {
        let (l, b) = ((30.0 * sx, 0.0), (xc * sx, 10.0 * sy));
        (l.0 + (b.0 - l.0) * 10.0 / 16.0, l.1 + (b.1 - l.1) * 10.0 / 16.0)
    };
    let line = |a: (f64, f64), b: (f64, f64)| Curve2::line(p2(a.0, a.1), p2(b.0, b.1));
    let curves = vec![
        line((-xc, -4.0), (xc, -4.0)),
        arc_between((xc, -10.0), 6.0, (xc, -4.0), touch(1.0, -1.0), false),
        arc_between((30.0, 0.0), 10.0, touch(1.0, -1.0), touch(1.0, 1.0), true),
        arc_between((xc, 10.0), 6.0, touch(1.0, 1.0), (xc, 4.0), false),
        line((xc, 4.0), (-xc, 4.0)),
        arc_between((-xc, 10.0), 6.0, (-xc, 4.0), touch(-1.0, 1.0), false),
        arc_between((-30.0, 0.0), 10.0, touch(-1.0, 1.0), touch(-1.0, -1.0), true),
        arc_between((-xc, -10.0), 6.0, touch(-1.0, -1.0), (-xc, -4.0), false),
    ];
    let plate = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::new(curves)]), 10.0)?;
    let boss = cylinder(k, p3(30.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 10.0, 30.0)?;
    union(k, plate, boss)
}
