//! A disc with a spherically crowned top and a thin obround vane standing on it, the loop where
//! the vane meets the crown rounded to radius 2.
//!
//! The disc: its section (the axis, the floor to radius 100, the rim up to z = 3, the sphere of
//! radius 800 about (0, 0, 3 − 100√63) back to the axis) revolved by the kernel. The vane: every
//! point within 2 of the segment from (22, 0) to (88, 0), from z = 1 to 31. Their union, then the
//! foot's four edges, by points on them (where each crosses the crown), rounded by the kernel's
//! fillet as one chain.
use crate::solids::boolean::union;
use crate::solids::outline::{deg, Chain};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_arc, make_line, revolve_profile_brep, AffineTransform, BrepSolid};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cz = 3.0 - 100.0 * 63f64.sqrt();
    let top = cz + 800.0;
    let crown = make_arc(p(0.0, 0.0, cz), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 800.0, (3.0 - cz).atan2(100.0), FRAC_PI_2)?;
    let section = vec![make_line(p(0.0, 0.0, top), p(0.0, 0.0, 0.0))?, make_line(p(0.0, 0.0, 0.0), p(100.0, 0.0, 0.0))?, make_line(p(100.0, 0.0, 0.0), p(100.0, 0.0, 3.0))?, crown];
    let disc = revolve_profile_brep(&section, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let mut plan = Chain::new();
    plan.line((22.0, -2.0), (88.0, -2.0))?
        .arc((88.0, 0.0), 2.0, deg(270.0), deg(450.0), true)?
        .line((88.0, 2.0), (22.0, 2.0))?
        .arc((22.0, 0.0), 2.0, deg(90.0), deg(270.0), true)?;
    let vane = extrude_profile_brep(&plan.curves, p(0.0, 0.0, 1.0), 30.0)?;
    let up = AffineTransform { elements: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0] };
    let vane = brep_kernel::transform_brep(&vane, up, false)?;
    let joined = union(&disc, &vane)?;
    let crown_at = |x: f64, y: f64| cz + (800.0f64 * 800.0 - x * x - y * y).sqrt();
    let foot: Vec<_> = plan.middles.iter().map(|m| p(m.x, m.y, crown_at(m.x, m.y))).collect();
    Ok(fillet_edges(&joined, &foot, None, 2.0, false, None)?)
}
