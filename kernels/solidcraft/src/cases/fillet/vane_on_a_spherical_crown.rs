use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::{FRAC_PI_2, PI};

/// The crown's sphere's radius.
const SPHERE: f64 = 800.0;

pub fn build() -> R<Body> {
    let line = |a: (f64, f64), b: (f64, f64)| Seg2::Line { a: v2(a.0, a.1), b: v2(b.0, b.1) };
    let c = 3.0 - (SPHERE * SPHERE - 100.0 * 100.0).sqrt();
    // the disc's half section in x and z: the floor, the rim, and the sphere from the rim's top to
    // the axis, anticlockwise about its centre
    let rim_top = (3.0 - c).atan2(100.0);
    let section = Loop2 {
        segs: vec![
            line((0.0, 0.0), (100.0, 0.0)),
            line((100.0, 0.0), (100.0, 3.0)),
            Seg2::Arc { center: v2(0.0, c), radius: SPHERE, start: rim_top, sweep: FRAC_PI_2 - rim_top },
            line((0.0, c + SPHERE), (0.0, 0.0)),
        ],
    };
    let disc = revolve_z(Region2 { outer: section, holes: vec![] }, 360.0)?;
    // the vane's plan: within 2 of the segment from (22, 0) to (88, 0)
    let obround = Loop2 {
        segs: vec![
            line((22.0, -2.0), (88.0, -2.0)),
            Seg2::Arc { center: v2(88.0, 0.0), radius: 2.0, start: -FRAC_PI_2, sweep: PI },
            line((88.0, 2.0), (22.0, 2.0)),
            Seg2::Arc { center: v2(22.0, 0.0), radius: 2.0, start: FRAC_PI_2, sweep: PI },
        ],
    };
    let vane = extrude(&Plane::XY, Region2 { outer: obround, holes: vec![] }, 1.0, 31.0)?;
    let joined = union(&disc, &vane)?;
    // the vane's foot, picked on its south side over x = 55 and followed all round
    let z = c + (SPHERE * SPHERE - 55.0 * 55.0 - 4.0).sqrt();
    let foot = solidcraft_kernel::tangent_chain(&joined, &[v3(55.0, -2.0, z)])?;
    solidcraft_kernel::fillet(&joined, &foot, 2.0)
}
