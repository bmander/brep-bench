//! `swept/box-screwed-inside-a-rod` a hundred times larger, built as it is at every length: the
//! rod of radius 500 from z = 0 to z = 2000, less what the box 250 ≤ x ≤ 350, −50 ≤ y ≤ 50,
//! 200 ≤ z ≤ 300 passes through as the screw rising 400 a turn carries it through 720°.
use crate::cases::swept::box_screwed_inside_a_rod::rod;
use crate::solids::*;

pub fn build() -> R<Body> {
    rod(100.0)
}
