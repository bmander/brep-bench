//! A square prism 0.001515 square about the z axis and 0.0015 tall, less the one 0.0015 square
//! between the same planes: a square tube with walls 0.0000075 thick.
use anvil_brep::Solid;

use crate::shared::{block, subtract};

pub fn build() -> Result<Solid, String> {
    let (a, b) = (0.001515 / 2., 0.0015 / 2.);
    subtract(&block([-a, -a, 0.], [a, a, 0.0015])?, &block([-b, -b, 0.], [b, b, 0.0015])?)
}
