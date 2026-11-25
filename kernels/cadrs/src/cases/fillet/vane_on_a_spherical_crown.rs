//! A disc (radius 100, flat beneath, its top the cap of the sphere of radius 800 through the rim's
//! top circle at z = 3) with a vane standing on it (the obround within 2 of the segment from
//! (22, 0) to (88, 0), from z = 1 to 31), the loop where the vane meets the crown rounded to
//! radius 2.
//!
//! The disc is the cylinder of radius 100 from z = 0 cut down to the ball of radius 800 about
//! (0, 0, 3 − 100√63), its top then that sphere exactly; the vane is its outline extruded, united
//! with the disc; then cadrs's Fillet on the edges where the vane's sides meet the sphere.
use crate::bench::{arc, ball, common, cylinder, extrude, p2, p3, union, v3, P3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = p3(0.0, 0.0, 3.0 - 100.0 * 63f64.sqrt());
    let drum = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 100.0, 20.0)?;
    let sphere = ball(k, c, 800.0)?;
    let disc = common(k, drum, sphere)?;
    let line = |a: (f64, f64), b: (f64, f64)| Curve2::line(p2(a.0, a.1), p2(b.0, b.1));
    let plan = vec![
        line((22.0, -2.0), (88.0, -2.0)),
        arc(p2(88.0, 0.0), 2.0, -90.0, 180.0),
        line((88.0, 2.0), (22.0, 2.0)),
        arc(p2(22.0, 0.0), 2.0, 90.0, 180.0),
    ];
    let vane = extrude(k, &Profile::of(Plane::xy(1.0), vec![Loop::new(plan)]), 30.0)?;
    let joined = union(k, disc, vane)?;
    // The foot: the edges on the sphere and on the vane's sides.
    let foot = |p: &P3| {
        ((p - c).norm() - 800.0).abs() < 1e-6 && (20.0 - 1e-6..=90.0 + 1e-6).contains(&p.x) && p.y.abs() <= 2.0 + 1e-6
    };
    let edges = k.edges_where(joined, &|e| foot(&e.start) && foot(&e.mid) && foot(&e.end))?;
    if edges.is_empty() {
        return Err(Refusal::from("no edge where the vane meets the crown"));
    }
    fillet(k, joined, &edges, 2.0)
}
