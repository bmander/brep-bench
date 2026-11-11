//! A block 20 by 12 by 5, from (−10, −6, 0) to (10, 6, 5), its four vertical edges rounded to
//! radius 3 (the rectangle with its corners rounded to radius 3, extruded 5 up the z axis), and
//! the rim of its top face rounded to radius 1: eight edges, four straight and four quarter
//! circles, each running on tangent into the next, rounded unbroken all the way round.
//!
//! The rounded rectangle is sketched on the Top plane (four lines and four arcs) and extruded 5;
//! one straight edge of the top's rim is picked, and the Fillet feature, radius 1, runs on along
//! its tangent chain round the whole rim.
use crate::bench::{arc, extrude, p2, p3};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (a, b, r) = (10.0, 6.0, 3.0);
    let outline = Loop::new(vec![
        Curve2::line(p2(-a + r, -b), p2(a - r, -b)),
        arc(p2(a - r, -b + r), r, -90.0, 90.0),
        Curve2::line(p2(a, -b + r), p2(a, b - r)),
        arc(p2(a - r, b - r), r, 0.0, 90.0),
        Curve2::line(p2(a - r, b), p2(-a + r, b)),
        arc(p2(-a + r, b - r), r, 90.0, 90.0),
        Curve2::line(p2(-a, b - r), p2(-a, -b + r)),
        arc(p2(-a + r, -b + r), r, 180.0, 90.0),
    ]);
    let block = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 5.0)?;
    let edge = k.edge_at(block, p3(0.0, -6.0, 5.0))?;
    fillet(k, block, &[edge], 1.0)
}
