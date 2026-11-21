//! The box from the origin to (10, 8, 6) hollowed to a wall 1 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{block, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let b = block([0., 0., 0.], [10., 8., 6.])?;
    shell_open(&b, &[], 1.)
}
