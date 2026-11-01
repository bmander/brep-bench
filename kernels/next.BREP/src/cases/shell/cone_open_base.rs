//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward to a wall 1 thick, open at its base: the cone less the cone of the same
//! half-angle whose apex is 5/3 lower.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cone = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 0.0, 8.0)?;
    let base = p(1.0, 1.0, 0.0);
    shell(&cone, &[base], 1.0)
}
