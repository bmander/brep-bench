//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, shelled inward to a wall 3.5 thick, open at its top: the side moved inward closes at (0, 0, 13/6), below the base moved up to z = 3.5, so nothing is hollowed and the result is the frustum itself.
use crate::bench::{cone, p3, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), V3::z(), 6.0, 3.0, 4.0)?;
    shell_inward(k, frustum, &[p3(1.0, 1.0, 4.0)], 3.5, Joins::Rounded)
}
