//! A sector of a ring about the z axis (radii 8 to 12, z from −1 to 1, polar angle 0° to 60°),
//! grooved by a torus (center (5, 0, 0), radii 2.5 and 2, its axis leaning 25° from z toward +y)
//! turned rigidly about the z axis from −140° to +140°.
//!
//! The block is its section (r from 8 to 12, z from −1 to 1) revolved 60° about z; the torus is
//! cadrs's torus primitive, its turn cadrs's sweep of a solid over part of a turn
//! (`Kernel::cut_swept`, a rigid turn about the z axis from −140° to +140°), cut from the block.
use crate::bench::{p3, revolve, torus, v3};
use crate::shared::swept::{cut_swept, swing, z_axis};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = Axis::new(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let ring = Profile::of(Plane::xz(0.0), vec![Loop::rectangle(8.0, -1.0, 12.0, 1.0)]);
    let block = revolve(k, &ring, z, Some(60.0))?;
    let (s, c) = 25f64.to_radians().sin_cos();
    let cutter = torus(k, p3(5.0, 0.0, 0.0), v3(0.0, s, c), 2.5, 2.0)?;
    cut_swept(k, block, cutter, &swing(z_axis(), -140.0, 140.0))
}
