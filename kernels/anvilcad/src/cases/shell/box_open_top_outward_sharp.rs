//! The box from the origin to (10, 8, 6) shelled outward 1 thick, open at its top, its edges sharp.
use anvil_brep::Solid;

use crate::shared::{block, shell_outward};

pub fn build() -> Result<Vec<Solid>, String> {
    let b = block([0., 0., 0.], [10., 8., 6.])?;
    shell_outward(&b, &[[5., 4., 6.]], 1.)
}
