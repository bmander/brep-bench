//! A cone frustum on the z axis, the disk of radius 18.6 at z = 0 up to the disk of radius 9.3 at
//! z = 12.4, bored through by a hole of radius 3.1, shelled inward to a wall 3.1 thick, open at its
//! top: the moved side and the moved bore meet at z = 34.1/3, below the top, so the cavity is a
//! closed ring and the top stays whole (`shell/bored-frustum-sealed-below-its-opening` with every
//! length 3.1 times as long).
use crate::bench::{cone, cut, cylinder, p3, V3};
use crate::shared::shell::{shell_inward, Joins};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let frustum = cone(k, p3(0.0, 0.0, 0.0), V3::z(), 18.6, 9.3, 12.4)?;
    // The drill runs past both faces.
    let drill = cylinder(k, p3(0.0, 0.0, -3.1), V3::z(), 3.1, 18.6)?;
    let bored = cut(k, frustum, drill)?;
    shell_inward(k, bored, &[p3(0.0, 6.2, 12.4)], 3.1, Joins::Rounded)
}
