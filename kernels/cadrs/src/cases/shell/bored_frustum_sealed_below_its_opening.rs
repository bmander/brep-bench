//! A cone frustum on the z axis, the disk of radius 6 at z = 0 up to the disk of radius 3 at z = 4, bored through by a hole of radius 1, shelled inward to a wall 1 thick, open at its top: the moved side and the moved bore meet at z = 11/3, below the top, so the cavity is a closed ring and the top stays whole.
use crate::bench::{cone, cut, cylinder, p3, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), V3::z(), 6.0, 3.0, 4.0)?;
    // The drill runs past both faces.
    let drill = cylinder(k, p3(0.0, 0.0, -1.0), V3::z(), 1.0, 6.0)?;
    let bored = cut(k, frustum, drill)?;
    shell_inward(k, bored, &[p3(0.0, 2.0, 4.0)], 1.0, Joins::Rounded)
}
