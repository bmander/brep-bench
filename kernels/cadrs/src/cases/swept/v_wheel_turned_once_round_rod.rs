//! A rod (the cylinder of radius 5 about the z axis from z = 0 to z = 20) grooved all round by a
//! wheel with a sharp V rim (the triangle with corners (4, 0, 10) and (6.5, 0, 10) ± 2.5 tan 30°·w
//! turned about the axis through (14, 0, 10) along w = (0, sin 20°, cos 20°)) turned a whole turn
//! about the z axis.
//!
//! The wheel is its triangle revolved; what it passes through is cadrs's sweep of a solid (turned
//! whole turns about the z axis: the solid of revolution it generates), cut from the rod.
use crate::bench::{cut, p3, revolve, v3};
use crate::shared::swept::rod;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Kernel, Loop, Plane, Profile, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = 20f64.to_radians();
    let (o, w) = (p3(14.0, 0.0, 10.0), v3(0.0, t.sin(), t.cos()));
    // Sketch x from the wheel's axis toward the rim's dip (−x), sketch y along w.
    let x = v3(-1.0, 0.0, 0.0);
    let on = Plane::new(o, x, x.cross(&w));
    let h = 2.5 * 30f64.to_radians().tan();
    let triangle = Loop::polygon(&[[10.0, 0.0], [7.5, -h], [7.5, h]]);
    let wheel = revolve(k, &Profile::of(on, vec![triangle]), Axis::new(o, w), None)?;
    let motion = SolidMotion::Turn { axis: crate::shared::swept::z_axis() };
    let swept = k.sweep_solid(wheel, &motion)?.first()?;
    let r = rod(k)?;
    cut(k, r, swept)
}
