//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circular edge around its top
//! chamfered with distance 5 on both faces, to a cone with its apex at (0, 0, 10).
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad cannot chamfer one edge of a curved solid: `Solid::chamfer` chamfers every edge and \
         only of an all-planar solid (`chamfer_all_edges_checked` refuses a non-planar face), and \
         `edge_blend` selects only edges between two planes, so the rim between the top and the \
         cylinder's side cannot be picked"
        .into())
}
