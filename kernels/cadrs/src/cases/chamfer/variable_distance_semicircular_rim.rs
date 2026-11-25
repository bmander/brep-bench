//! The half cylinder of radius 5 and height 6 on the side x ≥ 0, its curved rim chamfered by a
//! distance growing linearly with the angle round the axis, 1 to 2.
//!
//! cadrs's Chamfer has no distance that varies along its edge, so what the chamfer cuts off is
//! cut away instead. In the half plane at angle φ the chamfer is the line ρ + z = 11 − d(φ),
//! d = 3/2 + φ/π: the line at φ = 0 turned by φ about the z axis and lowered by φ/π. So the
//! corner beyond it is one triangle (its side on that line, run on past the top and the side by
//! more than it sinks, closed round the rim outside the part) carried by that screw: cadrs's
//! sweep along a helix falling 1/π a radian with the helix's own screw (`SweepControl::Helix`),
//! run 0.1 rad past each end of the rim, where there is no part to cut.
use crate::bench::{cut, p3, v3};
use crate::shared::fillet::half_cylinder;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Helix, Kernel, Loop, Plane, Profile, SweepControl, SweepSpec};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let half = half_cylinder(k, 5.0, 6.0)?;
    let pi = std::f64::consts::PI;
    let past = 0.1;
    let (phi0, angle) = (-pi / 2.0 - past, pi + 2.0 * past);
    let c0 = 11.0 - (1.5 + phi0 / pi); // ρ + z on the chamfer's line at φ0
    let radial = v3(phi0.cos(), phi0.sin(), 0.0);
    // The half plane at φ0: x along the radius, y up the axis.
    let plane = Plane {
        origin: p3(0.0, 0.0, 0.0),
        x_dir: nalgebra::Unit::new_normalize(radial),
        normal: nalgebra::Unit::new_normalize(radial.cross(&v3(0.0, 0.0, 1.0))),
    };
    // The triangle beyond the line, past the rim's top and side by more than the 1.06 it sinks.
    let e = 1.3;
    let triangle = Loop::polygon(&[[c0 - 6.0 - e, 6.0 + e], [5.0 + e, c0 - 5.0 - e], [5.0 + e, 6.0 + e]]);
    // The helix the rim's point at φ0 runs along: radius 5, falling angle/π over its turn.
    let path = Helix {
        axis: Axis::new(p3(0.0, 0.0, 6.0), v3(0.0, 0.0, 1.0)),
        x: nalgebra::Unit::new_normalize(v3(1.0, 0.0, 0.0)),
        r0: 5.0,
        r1: 5.0,
        height: -angle / pi,
        turns: angle / (2.0 * pi),
        start_angle: phi0,
    };
    let mut spec = SweepSpec::solid(path.path());
    spec.control = SweepControl::Helix(path);
    let beyond = k.sweep_with(&Profile::of(plane, vec![triangle]), &spec)?;
    let beyond = beyond.bodies.first().copied().ok_or("the sweep made no body")?;
    cut(k, half, beyond)
}
