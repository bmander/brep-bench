//! A pyramid on the rectangle from (−10, −6, 0) to (10, 6, 0) with its apex at (0, 0, 12), all
//! eight edges rounded to radius 3/2.
use anvil_brep::Solid;

use crate::shared::{fillet, pyramid};

pub fn build() -> Result<Solid, String> {
    let p = pyramid(&[(-10., -6.), (10., -6.), (10., 6.), (-10., 6.)], [0., 0., 12.])?;
    // the base's four edges and the four hips, each picked by its middle
    let edges = [
        [0., -6., 0.], [10., 0., 0.], [0., 6., 0.], [-10., 0., 0.],
        [-5., -3., 6.], [5., -3., 6.], [5., 3., 6.], [-5., 3., 6.],
    ];
    fillet(&p, &edges, 1.5)
}
