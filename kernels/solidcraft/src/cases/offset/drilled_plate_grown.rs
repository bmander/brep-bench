//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`): it moves the faces
//! along their normals and keeps the body's topology, each edge where its moved faces meet, sharp.
//! Grown by every point within 1, the plate's outer edges and corners round and the hole's rims
//! become saddles of tori, and the kernel has no offset that rounds its joins.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset with rounded joins: its offset of a solid, \
         `offset_faces`, moves the faces and joins them sharp where they meet, so the grown \
         plate's edges, corners and hole rims would not be rounded"
        .into())
}
