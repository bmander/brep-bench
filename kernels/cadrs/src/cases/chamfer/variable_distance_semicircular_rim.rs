//! The half cylinder of radius 5 and height 6 on the side x ≥ 0, its curved rim chamfered by the
//! same distance on both faces, growing linearly with the angle round the axis from 1 to 2,
//! d = 3/2 + φ/π: the ruled surface between a spiral of Archimedes on the top and a helix on the
//! side.
//!
//! cadrs's variable chamfer (`Kernel::chamfer_variable`): its distance laws run from the edge's
//! start to its end, which on the rim is by angle, so each end gets the distance at its angle.
use crate::bench::p3;
use crate::shared::fillet::half_cylinder;
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferLaw, DefaultKernel, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let half = half_cylinder(k, 5.0, 6.0)?;
    let rim = k.edge_at(half, p3(5.0, 0.0, 6.0))?;
    let info = k.edges(half)?.into_iter().find(|e| e.id == rim).ok_or("the rim is gone")?;
    // d = 3/2 + φ/π: 1 at φ = −90° (y < 0), 2 at φ = 90°
    let d = |y: f64| if y < 0.0 { 1.0 } else { 2.0 };
    let law = vec![(0.0, d(info.start.y)), (1.0, d(info.end.y))];
    let chamfer = ChamferLaw { edge: rim, face: None, first: law.clone(), second: law };
    Ok(k.chamfer_variable(half, &[chamfer])?.first()?)
}
