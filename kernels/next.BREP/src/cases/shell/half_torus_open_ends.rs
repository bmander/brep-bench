//! The half with y >= 0 of the solid torus about the z axis whose tube has radius 2 about a middle
//! circle of radius 6, shelled inward to a wall 0.5 thick, open at both flat ends: a curved pipe,
//! the half torus less the half torus of tube radius 1.5.
//!
//! The half torus is the tube's disc, in the xz plane about (6, 0, 0), revolved half a turn about
//! the z axis from +x towards +y; its circle is given as two half circles, the revolve taking a
//! profile of at least two curves.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_arc, revolve_profile_brep, BrepSolid};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let (centre, x, z) = (p(6.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    let tube = [make_arc(centre, x, z, 2.0, 0.0, PI)?, make_arc(centre, x, z, 2.0, PI, 2.0 * PI)?];
    let half = revolve_profile_brep(&tube, p(0.0, 0.0, 0.0), z, PI)?;
    let ends = [p(6.5, 0.0, 0.5), p(-6.5, 0.0, 0.5)];
    shell(&half, &ends, 0.5)
}
