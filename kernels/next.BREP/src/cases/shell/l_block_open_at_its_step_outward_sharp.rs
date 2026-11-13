//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp.
use crate::solids::shell::shell;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(20.0, 0.0, 0.0), p(20.0, 20.0, 0.0),
        p(15.0, 20.0, 0.0), p(15.0, 5.0, 0.0), p(0.0, 5.0, 0.0),
    ])?;
    let block = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 20.0)?;
    shell(&block, &[p(7.5, 5.0, 10.0)], -1.0)
}
