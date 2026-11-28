//! `loft/square-to-round` three times larger, built as it is: the square of side 120 on z = 0 to
//! the circle of radius 45 on z = 90.
use super::square_to_round::{Sizes, transition};
use crate::solids::*;

pub fn build() -> R<Body> {
    transition(Sizes { half: 60.0, radius: 45.0, height: 90.0 })
}
