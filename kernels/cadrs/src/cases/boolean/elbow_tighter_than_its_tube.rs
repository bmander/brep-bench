//! A pipe elbow whose bend is tighter than the pipe is thick: the disk of radius 20 about
//! (0, 0, 18) in the plane x = 0 turned a quarter turn about the y axis (+z to +x), with a straight
//! leg of radius 20 and length 40 at each end (about y = 0, z = 18 from x = −40 to 0, and about
//! x = 18, y = 0 from z = −40 to 0), united.
//!
//! The disk reaches 2 past the bend axis. Its part past the axis turns into the corner where the
//! first leg already is (a point (0, y, z) with z < 0 turns to (z sin θ, y, z cos θ), as far from
//! the leg's axis as it was or nearer, x ≤ 0), so the turn is the revolve of the disk's part on
//! the near side of the axis: cadrs's revolve of that region, a quarter turn about the y axis.
//! The legs are circles extruded; the three united.
use crate::bench::{arc, cylinder, p2, p3, revolve, union_each, v3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // The plane x = 0, its first axis y and its second z.
    let plane = Plane::new(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0), v3(1.0, 0.0, 0.0));
    let half = (400f64 - 324.0).sqrt();
    let start = (-18f64).atan2(half).to_degrees();
    let sweep = 360.0 - 2.0 * (90.0 + start);
    let near = Loop::new(vec![
        arc(p2(0.0, 18.0), 20.0, start, sweep),
        Curve2::line(p2(-half, 0.0), p2(half, 0.0)),
    ]);
    let turn = revolve(k, &Profile::of(plane, vec![near]), Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 1.0, 0.0)), Some(90.0))?;
    let first = cylinder(k, p3(-40.0, 0.0, 18.0), v3(1.0, 0.0, 0.0), 20.0, 40.0)?;
    let second = cylinder(k, p3(18.0, 0.0, -40.0), v3(0.0, 0.0, 1.0), 20.0, 40.0)?;
    union_each(k, turn, &[first, second])
}
