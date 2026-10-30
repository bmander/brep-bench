//! A ball of radius 10 at the origin joined to a post of radius 4 on the z axis from z = 0 to
//! z = 15, the circle where the post leaves the ball rounded to radius 2, adding material.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad cannot round one chosen edge of a curved solid: `edge_blend` selects only edges \
         between two planes, and `Solid::fillet` rounds every edge (here the post's top rim as \
         well); the curved per-edge pipeline under it (`fillet_edges_detailed`) insets every face \
         by the radius, which is right only when every edge is rounded"
        .into())
}
