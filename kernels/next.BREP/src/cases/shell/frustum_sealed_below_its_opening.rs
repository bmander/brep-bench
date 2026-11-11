//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, shelled inward to a wall 2.7 thick, open at its top: the side moved inward closes to its apex at (0, 0, 3.5), below the top, so the cavity is a closed cone and the top stays whole.
use crate::solids::p;
use crate::solids::shell::shell;
use crate::Refusal;
use brep_kernel::{make_cone_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let frustum = make_cone_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 6.0, 3.0, 4.0)?;
    let top = p(1.0, 1.0, 4.0);
    shell(&frustum, &[top], 2.7)
}
