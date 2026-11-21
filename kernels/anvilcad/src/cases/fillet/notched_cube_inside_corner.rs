//! A cube from the origin to (20, 20, 20) less the cube from (10, 10, 10) to (20, 20, 20), the
//! notch's three inside edges rounded to radius 3 all at once.
use anvil_brep::Solid;

use crate::shared::{block, fillet, subtract};

pub fn build() -> Result<Solid, String> {
    let notched = subtract(&block([0., 0., 0.], [20., 20., 20.])?, &block([10., 10., 10.], [21., 21., 21.])?)?;
    fillet(&notched, &[[15., 10., 10.], [10., 15., 10.], [10., 10., 15.]], 3.)
}
