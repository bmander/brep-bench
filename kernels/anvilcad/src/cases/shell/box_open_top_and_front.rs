//! The box from the origin to (10, 8, 6) shelled inward 1 thick, open at its top and its front y = 0.
use anvil_brep::Solid;

use crate::shared::{block, shell_open};

pub fn build() -> Result<Vec<Solid>, String> {
    let b = block([0., 0., 0.], [10., 8., 6.])?;
    shell_open(&b, &[[5., 4., 6.], [5., 0., 3.]], 1.)
}
