//! Two blocks 10 high on the xy plane, spanning x and y from 0 to 10 and from 5 to 15: their difference, the second cut from the first.
use anvil_brep::Solid;

use crate::shared::{block, subtract};

pub fn build() -> Result<Solid, String> {
    let a = block([0., 0., 0.], [10., 10., 10.])?;
    let b = block([5., 5., 0.], [15., 15., 10.])?;
    subtract(&a, &b)
}
