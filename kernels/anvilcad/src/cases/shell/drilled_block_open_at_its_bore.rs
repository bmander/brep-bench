//! The block from the origin to (20, 20, 10) bored through along z by a hole of radius 3 at (10, 10), shelled inward 1 thick, open at the bore.
use anvil_brep::Solid;

use crate::shared::{at, block, cylinder, shell_open, subtract};

pub fn build() -> Result<Vec<Solid>, String> {
    let d = subtract(&block([0., 0., 0.], [20., 20., 10.])?, &cylinder(&at([10., 10., 0.]), 3., -1., 11.)?)?;
    shell_open(&d, &[[10., 13., 5.]], 1.)
}
