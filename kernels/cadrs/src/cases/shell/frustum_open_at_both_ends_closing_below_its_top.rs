//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, shelled inward to a wall 2.7 thick, open at both ends: the side moved inward meets the base's plane in the circle of radius 2.625 and closes to its apex at (0, 0, 3.5), below the top, so the cavity is a cone open at the base only and the top stays whole.
use crate::bench::{cone, p3, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), V3::z(), 6.0, 3.0, 4.0)?;
    shell_inward(k, frustum, &[p3(1.0, 1.0, 0.0), p3(1.0, 1.0, 4.0)], 2.7, Joins::Rounded)
}
