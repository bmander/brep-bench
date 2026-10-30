//! A plate from (−20, −20, 0) to (20, 20, 4) with a boss of radius 5 on the z axis from z = 4 to
//! z = 8, the boss's top rim and its root both rounded to radius 2, so its profile is an S.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad cannot round chosen edges of a curved solid: `edge_blend` selects only edges \
         between two planes, and `Solid::fillet` rounds every edge (here the plate's twelve as \
         well); and every blend refuses a solid whose faces have inner loops \
         (`BlendError::InnerLoops`), as the plate's top has round the boss's root once they are \
         joined"
        .into())
}
