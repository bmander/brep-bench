//! `swept/torus-screwed-through-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: a rod (radius 5, from the origin
//! to 20w) grooved by a wheel (a torus of radii 6 and 2 about 11.5u, its axis cos h·v − sin h·w,
//! h = atan(2π·5.5/30)) screwed about the rod's axis, rising 30 a turn, from −108° to 348°.
//!
//! The wheel is cadrs's torus primitive and the rod a circle extruded; cadrs's cut by a swept
//! solid.
use crate::bench::{cylinder, p3, torus, v3};
use crate::shared::swept::{cut_swept, screw};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let (u, v, w) = (v3(2.0, -2.0, 1.0) / 3.0, v3(2.0, 1.0, -2.0) / 3.0, v3(1.0, 2.0, 2.0) / 3.0);
    let o = p3(0.0, 0.0, 0.0);
    let h = (std::f64::consts::TAU * 5.5 / 30.0).atan();
    let wheel = torus(k, o + u * 11.5, v * h.cos() - w * h.sin(), 6.0, 2.0)?;
    let r = cylinder(k, o, w, 5.0, 20.0)?;
    cut_swept(k, r, wheel, &screw(Axis::new(o, w), 30.0, -108.0, 348.0))
}
