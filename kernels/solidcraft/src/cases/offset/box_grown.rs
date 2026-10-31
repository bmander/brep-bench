//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`, which the app's
//! outward Shell builds on too): it moves the faces along their normals and keeps the body's
//! topology, so each edge goes to where its moved faces meet, sharp. Grown by every point within
//! 1, a box's edges become quarter cylinders and its corners eighths of a ball, and the kernel has
//! no offset that rounds its joins.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset with rounded joins: its offset of a solid, \
         `offset_faces`, moves the faces and joins them sharp where they meet, so the grown box's \
         edges and corners would not be rounded"
        .into())
}
