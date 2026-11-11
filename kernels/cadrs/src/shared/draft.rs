//! What the cadrs implementations of the draft cases share: cadrs's Draft feature, and picking
//! a part's side faces.
use crate::bench::{P3, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, DraftSpec, FaceId, FaceInfo, Kernel, SurfaceKind};

/// `body` with `faces` drafted `angle` degrees for the pull direction `pull` about the neutral
/// plane through `neutral` square to the pull (cadrs's Draft feature, tangent propagation on).
/// A positive angle leans the faces into the material as they run along the pull (a boss
/// narrows as it rises, a hole widens); a negative one leans them out.
pub fn drafted(
    k: &mut DefaultKernel,
    body: BodyId,
    faces: Vec<FaceId>,
    angle: f64,
    pull: V3,
    neutral: P3,
) -> Result<BodyId, Refusal> {
    Ok(k.draft(body, &DraftSpec::new(faces, angle.to_radians(), pull, neutral))?.first()?)
}

/// Whether a face is a side for a pull along z: a plane square to the xy plane, or a cylinder
/// or cone about a vertical axis.
pub fn is_side(f: &FaceInfo) -> bool {
    match f.kind {
        SurfaceKind::Plane => f.plane.is_some_and(|p| p.normal.z.abs() < 1e-9),
        SurfaceKind::Cylinder | SurfaceKind::Cone => {
            f.axis.is_some_and(|a| a.dir.x.abs() < 1e-9 && a.dir.y.abs() < 1e-9)
        }
        _ => false,
    }
}
