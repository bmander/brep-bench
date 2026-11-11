//! A hypoid gear's tooth space cut by its pinion.
//!
//! geop sweeps a profile along a path, turning it with the path; it has no sweep of a solid
//! through a motion, and the space the pinion's tooth cuts is bounded by the envelope of its
//! flanks as the two skew shafts turn, which geop cannot build.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop sweeps a profile along a path, turning it with the path; it has no sweep of a \
         solid through a motion, and the space the pinion's tooth cuts is bounded by the \
         envelope of its flanks as the two skew shafts turn, which geop cannot build"
        .into())
}
