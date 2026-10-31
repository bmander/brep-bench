//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`): it moves the faces
//! along their normals and keeps the body's topology, each edge where its moved faces meet, sharp.
//! Shrunk by 1, the flange's faces cross each other and must be taken away, and where the flange
//! met the drum its concave edges become pieces of tori meeting in a ridge: the kernel's offset
//! neither removes faces moved through each other nor rounds a join.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset that removes faces moved through each other, nor one \
         with rounded joins: its offset of a solid, `offset_faces`, keeps the body's topology and \
         joins the moved faces sharp, so the flange could not vanish nor its inside edges become \
         tori"
        .into())
}
