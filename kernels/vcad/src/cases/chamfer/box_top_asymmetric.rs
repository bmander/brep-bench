//! A box from the origin to (20, 10, 6) with the four edges around its top face chamfered,
//! distance 1 across the top and 2 down the sides.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no chamfer with two distances: `Solid::chamfer` and `edge_blend`'s \
         `BlendSection` take one `size`, the same setback on both faces of the edge; and \
         `edge_blend` blends no two edges that share a vertex (`apply_edge_blend` skips them), so \
         the four edges of a ring cannot be chamfered together"
        .into())
}
