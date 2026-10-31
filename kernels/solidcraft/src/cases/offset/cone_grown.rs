//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`): it moves the faces
//! along their normals and keeps the body's topology, each edge where its moved faces meet. Grown
//! by every point within 1, the cone's rim becomes a piece of a torus and its apex a cap of a ball,
//! and the kernel has no offset that rounds its joins or caps a point.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset with rounded joins: its offset of a solid, \
         `offset_faces`, moves the faces and joins them sharp where they meet, so the grown \
         cone's rim would not become a torus nor its apex a ball's cap"
        .into())
}
