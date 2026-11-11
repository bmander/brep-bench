//! A cone frustum on the z axis, the disk of radius 7 at z = 0 up to the disk of radius 1 at z = 8, shelled inward to a wall 2 thick, open at its top: the side moved inward closes to its apex at (0, 0, 6), 2 below the top, so the cavity is a closed cone and the top stays whole.
use crate::bench::{cone, p3, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), V3::z(), 7.0, 1.0, 8.0)?;
    shell_inward(k, frustum, &[p3(0.5, 0.5, 8.0)], 2.0, Joins::Rounded)
}
