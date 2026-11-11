//! What the cadrs implementations of the edit cases share: Move face, by a rigid motion or by an
//! offset along the faces' normals.
use crate::bench::V3;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, FaceId, Kernel, Motion, OffsetSpec};

/// `body` with `faces` moved by the translation `v` (Move face, Translate), the faces round them
/// extended or trimmed to meet them.
pub fn move_faces(
    k: &mut DefaultKernel,
    body: BodyId,
    faces: &[FaceId],
    v: V3,
) -> Result<BodyId, Refusal> {
    Ok(k.move_faces(body, faces, &Motion::translation(v))?.first()?)
}

/// `body` with `face` moved `distance` along its outward normal (Move face, Offset; negative
/// moves it into the material), the other faces kept and trimmed to meet it.
pub fn offset_face(
    k: &mut DefaultKernel,
    body: BodyId,
    face: FaceId,
    distance: f64,
) -> Result<BodyId, Refusal> {
    offset_faces(k, body, &[face], distance)
}

/// `body` with each of `faces` moved `distance` along its outward normal at once (Move face,
/// Offset; negative moves them into the material), the other faces kept and trimmed to meet them.
pub fn offset_faces(
    k: &mut DefaultKernel,
    body: BodyId,
    faces: &[FaceId],
    distance: f64,
) -> Result<BodyId, Refusal> {
    let moved: Vec<_> = faces.iter().map(|&f| (f, distance)).collect();
    Ok(k.offset(body, &OffsetSpec::of_faces(&moved).mitred())?.first()?)
}
