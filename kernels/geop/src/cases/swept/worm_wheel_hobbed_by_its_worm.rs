//! A worm wheel hobbed by its worm.
//!
//! geop sweeps a profile along a path, turning it with the path; it has no sweep of a solid
//! through a motion, and the worm wheel's teeth are bounded by the envelope of the worm's thread
//! as both turn, which geop cannot build.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop sweeps a profile along a path, turning it with the path; it has no sweep of a \
         solid through a motion, and the worm wheel's teeth are bounded by the envelope of the \
         worm's thread as both turn, which geop cannot build"
        .into())
}
