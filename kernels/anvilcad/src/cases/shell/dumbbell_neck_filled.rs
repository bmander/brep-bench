//! A dumbbell plate shelled open at its top, the wall every point within 1.5 of its faces.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep's shell moves each face by the thickness and extends the moved faces until they meet, in sharp edges; it has no shell whose wall is every point within the thickness, rounded where the moved faces part".into())
}
