//! The solid a torus (center (3, 0, 2), axis along z, radii 1 and 0.5) fills as it turns by t/4
//! about the vertical line through (2, 0, 0), seen from a frame turning by t about the line
//! through (0, 0.5, 0.5) along x, as t runs from −75° to 75°.
//!
//! The torus is cadrs's torus primitive; cadrs's `Kernel::sweep_solid` with the rigid motion of
//! its two turns (by t/4 about the first line, then by −t about the second) gives what it fills.
//! The kernel builds it exactly: the sheets of the torus's envelope (its core circle's sweep, a
//! tube radius either side), trimmed where the motion covers them and fitted as B-spline faces,
//! closed by the torus where the motion starts and where it stops.
use crate::bench::{p3, torus, v3};
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Kernel, RigidStep, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = torus(k, p3(3.0, 0.0, 2.0), v3(0.0, 0.0, 1.0), 1.0, 0.5)?;
    let (a0, a1) = ((-75f64).to_radians(), 75f64.to_radians());
    let motion = SolidMotion::Rigid {
        steps: vec![
            RigidStep::Turn { axis: Axis::new(p3(2.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), from: a0 / 4.0, to: a1 / 4.0 },
            RigidStep::Turn { axis: Axis::new(p3(0.0, 0.5, 0.5), v3(1.0, 0.0, 0.0)), from: -a0, to: -a1 },
        ],
    };
    let swept = k.sweep_solid(t, &motion)?;
    swept.bodies.first().copied().ok_or_else(|| Refusal::from("the sweep made no body"))
}
