//! A ball of radius 5 about the origin hollowed to a wall 1 thick with no opening.
use anvil_brep::Solid;

use crate::shared::{ball, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    shell_open(&ball([0., 0., 0.], 5.)?, &[], 1.)
}
