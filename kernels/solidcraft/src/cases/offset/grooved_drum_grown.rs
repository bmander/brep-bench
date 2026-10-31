//! Refused. The kernel's one offset of a solid is Offset Face (`offset_faces`): it moves the faces
//! along their normals and keeps the body's topology, each edge where its moved faces meet, sharp.
//! Grown by 1, the drum's rims become quarter tori, the groove's walls cross each other and must
//! be taken away, and its outer rims become pieces of tori meeting in a crease: the kernel's
//! offset neither rounds a join nor removes faces moved through each other.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    Err("solidcraft-kernel has no offset with rounded joins, nor one that removes faces moved \
         through each other: its offset of a solid, `offset_faces`, keeps the body's topology and \
         joins the moved faces sharp, so the drum's rims would not round nor the groove close"
        .into())
}
