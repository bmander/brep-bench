//! An L-shaped block shelled outward open at its step, the walls beyond the opening kept. The
//! outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
//! shelled outward to a wall 1 thick, open at the face y = 5, its edges sharp: the polygon through
//! (−1, −1), (21, −1), (21, 21), (14, 21), (14, 5) and (−1, 5), from z = −1 to 21, less the block.
//!
//! The block is the outline on the Top plane extruded 20; cadrs's Shell feature, outward, its
//! corners sharp, the face y = 5 picked as the face to remove.
use crate::bench::{extrude, p3};
use crate::shared::shell::{shell_outward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outline = Loop::polygon(&[
        [0.0, 0.0],
        [20.0, 0.0],
        [20.0, 20.0],
        [15.0, 20.0],
        [15.0, 5.0],
        [0.0, 5.0],
    ]);
    let block = extrude(k, &Profile::of(Plane::xy(0.0), vec![outline]), 20.0)?;
    shell_outward(k, block, &[p3(7.5, 5.0, 10.0)], 1.0, Joins::Sharp)
}
