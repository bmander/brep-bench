//! `swept/cone-spun-about-a-line-through-it` ten times larger, built as it is at every length:
//! the cone from its point at −20w to its base at 50w, of radius 70 tan 25°, spun 150° about the
//! z axis.
use crate::cases::swept::cone_spun_about_a_line_through_it::spun;
use crate::solids::*;

pub fn build() -> R<Body> {
    spun(10.0)
}
