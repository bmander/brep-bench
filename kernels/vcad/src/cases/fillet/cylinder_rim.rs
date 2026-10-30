//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circular edge around its top
//! rounded to radius 1.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad cannot round one chosen edge of a curved solid: `edge_blend` selects only edges \
         between two planes, and `Solid::fillet` rounds every edge (here both rims, and on a \
         cylinder it declines as diverged); the curved per-edge pipeline under it \
         (`fillet_edges_detailed`) insets every face by the radius, which is right only when \
         every edge is rounded"
        .into())
}
