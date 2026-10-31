//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`): it moves the faces
//! along their normals and keeps the body's topology, each edge where its moved faces meet, sharp.
//! Shrunk by every point at least 1 inside, the plate's concave edge becomes a concave quarter
//! cylinder, and the kernel has no offset that rounds its joins.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset with rounded joins: its offset of a solid, \
         `offset_faces`, moves the faces and joins them sharp where they meet, so the shrunk \
         plate's concave edge would not be rounded"
        .into())
}
