//! A plate from (−10, −10, 0) to (10, 10, 2) with a hole of radius 3 on the z axis, both of the
//! hole's rims rounded to radius 1, so the hole's wall is the inner half of a torus.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad cannot round chosen edges of a curved solid: `edge_blend` selects only edges \
         between two planes, and `Solid::fillet` rounds every edge (here the plate's twelve as \
         well); and every blend refuses a solid whose faces have inner loops \
         (`BlendError::InnerLoops`), as the plate's top and bottom have round the hole"
        .into())
}
