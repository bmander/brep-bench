//! `swept/box-screwed-inside-a-rod` turned rigidly about the origin so that the rod's axis is
//! w = (1, 2, 2)/3: the point that case puts at (x, y, z) is here at x·u + y·v + z·w, with
//! u = (2, −2, 1)/3 and v = (2, 1, −2)/3.
//!
//! Built as that case builds it, the box's sweep made by elementary geometry from the kernel's
//! helical sweep of planar regions (solidcraft-kernel has no sweep of a solid), each piece turned
//! into place as it is made, so that every cut is made about the tilted axis.
use super::box_screwed_inside_a_rod::placed_rod;
use crate::solids::*;
use solidcraft_kernel::transform_matrix;

/// The turn taking x, y and z to u, v and w, column by column.
const TURN: [[f64; 4]; 4] = [
    [2.0 / 3.0, -2.0 / 3.0, 1.0 / 3.0, 0.0],
    [2.0 / 3.0, 1.0 / 3.0, -2.0 / 3.0, 0.0],
    [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn build() -> R<Body> {
    placed_rod(1.0, &|piece| transform_matrix(&piece, TURN))
}
