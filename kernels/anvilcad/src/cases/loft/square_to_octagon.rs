//! A transition piece from the square −20 ≤ x, y ≤ 20 at z = 0 to the octagon at z = 30 with
//! corners (±6, ±14) and (±14, ±6): each side of the square joined to the octagon's side parallel
//! to it by a flat trapezoid, each corner to the cut side facing it by a flat triangle.
use anvil_brep::Solid;

use crate::shared::{beyond, block, cut_all};

pub fn build() -> Result<Solid, String> {
    // anvil-brep's ruled loft matches the two loops by arc length, and pairs no corner with a whole
    // side. The walls are all flat, so the solid is the block between the two sections cut by
    // their planes: the trapezoid through (±20, −20, 0) and (±6, −14, 30), normal (0, −5, 1), and
    // the triangle through (20, −20, 0), (6, −14, 30) and (14, −6, 30), normal (3, −3, 2), and the
    // others turned a quarter turn at a time
    let mut walls = vec![
        beyond([0., 20., 0.], [0., 5., 1.], 100.),
        beyond([0., -20., 0.], [0., -5., 1.], 100.),
        beyond([20., 0., 0.], [5., 0., 1.], 100.),
        beyond([-20., 0., 0.], [-5., 0., 1.], 100.),
    ];
    for (sx, sy) in [(1., 1.), (-1., 1.), (-1., -1.), (1., -1.)] {
        walls.push(beyond([20. * sx, 20. * sy, 0.], [3. * sx, 3. * sy, 2.], 100.));
    }
    cut_all(block([-20., -20., 0.], [20., 20., 30.])?, walls)
}
