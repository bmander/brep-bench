//! A cube of side 10 from the origin, and a second standing on half of its top, joined into one.
use anvil_brep::Solid;

use crate::shared::{block, union};

pub fn build() -> Result<Solid, String> {
    let a = block([0., 0., 0.], [10., 10., 10.])?;
    let b = block([5., 0., 10.], [15., 10., 20.])?;
    union(&a, &b)
}
