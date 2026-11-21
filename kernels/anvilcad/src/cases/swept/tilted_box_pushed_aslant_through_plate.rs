//! A plate from (−15, −15, 0) to (15, 15, 4) less what a tilted 4 × 3 × 2 box passes through as it
//! slides along (4, 2, 18) from below the plate to above it.
use anvil_brep::Solid;

use crate::shared::{block, subtract, v3, zonotope};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. A box slid along a line fills the zonotope of its three
    // edges and the slide, centered halfway along
    let (e1, e2, e3) = (v3(2., 1., 2.) / 3., v3(-2., 2., 1.) / 3., v3(1., 2., -2.) / 3.);
    let slide = v3(4., 2., 18.);
    let center = v3(-2., -1., -8.) + slide * 0.5;
    let swept = zonotope(center, &[e1 * 4., e2 * 3., e3 * 2., slide])?;
    subtract(&block([-15., -15., 0.], [15., 15., 4.])?, &swept)
}
