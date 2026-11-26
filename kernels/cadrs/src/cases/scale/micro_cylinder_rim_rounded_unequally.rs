//! A cylinder of radius 0.05 on the z axis from z = 0 to z = 0.1, its top rim rounded unequally:
//! 0.01 across the top and 0.03 down the side, a quarter ellipse in section
//! (`fillet/elliptic-section-cylinder-rim` at a hundredth of its size).
//!
//! The cylinder is a circle extruded; its rim is rounded by cadrs's Fillet with an asymmetric
//! (two-distance) section, 0.01 on the top and 0.03 on the side.
use crate::bench::{cylinder, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, FilletProfile, FilletSpec, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 0.05, 0.1)?;
    let rim = k.edge_at(c, p3(0.05, 0.0, 0.1))?;
    let top = k.face_at(c, p3(0.0, 0.0, 0.1))?;
    let info = k.edges(c)?.into_iter().find(|e| e.id == rim).ok_or("the rim is gone")?;
    // 0.01 on the top and 0.03 on the side: flipped where the top is not the edge's first face
    let flip = info.faces[0] != Some(top);
    let profile = FilletProfile::Asymmetric { second: 0.03, flip };
    let spec = FilletSpec { profile, ..FilletSpec::radius(0.01) };
    Ok(k.fillet_with(c, &[rim], &spec)?.first()?)
}
