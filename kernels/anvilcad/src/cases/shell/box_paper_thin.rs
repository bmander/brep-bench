//! The box from the origin to (10, 8, 6) shelled inward 0.001 thick, open at its top.
use anvil_brep::Solid;

use crate::shared::{block, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let b = block([0., 0., 0.], [10., 8., 6.])?;
    shell_open(&b, &[[5., 4., 6.]], 0.001)
}
