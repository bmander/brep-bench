//! A spiroid face gear hobbed by its tapered worm.
//!
//! geop sweeps a profile along a path, turning it with the path; it has no sweep of a solid
//! through a motion, and the face gear's teeth are bounded by the envelope of the worm's thread as
//! both turn, which geop cannot build.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop sweeps a profile along a path, turning it with the path; it has no sweep of a \
         solid through a motion, and the face gear's teeth are bounded by the envelope of the \
         worm's thread as both turn, which geop cannot build"
        .into())
}
