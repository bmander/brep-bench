//! A block from z = 0 to z = 10 over a plan of thirteen tangent pieces (an 80 by 40 rectangle
//! with its corners rounded to 5, a lobe of radius 10 hanging from its south side between a dip
//! and a straight rise into a reverse arc), its top rim rounded to radius 2.
//!
//! The plan is the thirteen pieces as the case lists them, extruded 10; then cadrs's Fillet on
//! the thirteen edges of its top rim.
use crate::bench::{arc, extrude, p2};
use crate::shared::fillet::{edges_at_height, fillet};
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let line = |a: (f64, f64), b: (f64, f64)| Curve2::line(p2(a.0, a.1), p2(b.0, b.1));
    let curves = vec![
        line((5.0, 0.0), (30.0, 0.0)),
        arc(p2(30.0, -5.0), 5.0, 90.0, -90.0),
        arc(p2(45.0, -5.0), 10.0, 180.0, 180.0),
        line((55.0, -5.0), (55.0, 0.0)),
        arc(p2(60.0, 0.0), 5.0, 180.0, -90.0),
        line((60.0, 5.0), (75.0, 5.0)),
        arc(p2(75.0, 10.0), 5.0, -90.0, 90.0),
        line((80.0, 10.0), (80.0, 35.0)),
        arc(p2(75.0, 35.0), 5.0, 0.0, 90.0),
        line((75.0, 40.0), (5.0, 40.0)),
        arc(p2(5.0, 35.0), 5.0, 90.0, 90.0),
        line((0.0, 35.0), (0.0, 5.0)),
        arc(p2(5.0, 5.0), 5.0, 180.0, 90.0),
    ];
    let block = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::new(curves)]), 10.0)?;
    let rim = edges_at_height(k, block, 10.0)?;
    fillet(k, block, &rim, 2.0)
}
