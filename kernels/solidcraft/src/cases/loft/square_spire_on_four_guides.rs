use super::slab_pinched_on_four_guides::on_guides;
use crate::solids::*;

/// The spire's four ridges, sewn as the pinched slab's guides are: each wall the ruled surface
/// between two ridges, closing to nothing at the apex where they meet, on a flat square floor.
pub fn build() -> R<Body> {
    on_guides([[9.0, 9.0, 0.0], [15.0, 15.0, 8.0], [12.0, 12.0, 19.0], [0.0, 0.0, 24.0]])
}
