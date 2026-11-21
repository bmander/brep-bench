//! A plate from (0, −5, 0) to (20, 5, 4) less what a square bar on its corner (6 ≤ x ≤ 14,
//! |y| + |z + 6| ≤ 1) passes through, slid 8 along (0.3, 0, √0.91) into it.
use anvil_brep::Solid;

use crate::shared::{block, subtract, v3, zonotope};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. The bar is a parallelepiped, its edges (8, 0, 0),
    // (0, 1, 1) and (0, −1, 1); slid along a line it fills the zonotope of those and the slide
    let slide = v3(0.3, 0., 0.91f64.sqrt()) * 8.;
    let center = v3(10., 0., -6.) + slide * 0.5;
    let swept = zonotope(center, &[v3(8., 0., 0.), v3(0., 1., 1.), v3(0., -1., 1.), slide])?;
    subtract(&block([0., -5., 0.], [20., 5., 4.])?, &swept)
}
