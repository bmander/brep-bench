//! A pipe of radius 2 swept along three edges of the cube of side 30, from the origin to
//! (30, 0, 0), to (30, 30, 0), to (30, 30, 30), mitered at its corners.
use anvil_brep::Solid;

use crate::shared::{beyond, rod, subtract, union};

pub fn build() -> Result<Solid, String> {
    // anvil-brep's sweep turns no sharp corner: each run is its disk swept straight along it, run
    // past its corners, and cut by the planes that bisect them (x + y = 30, y + z = 30)
    let first = subtract(&rod([0., 0., 0.], [40., 0., 0.], 2.)?, &beyond([30., 0., 0.], [1., 1., 0.], 100.)?)?;
    let middle = rod([30., -10., 0.], [30., 40., 0.], 2.)?;
    let middle = subtract(&middle, &beyond([30., 0., 0.], [-1., -1., 0.], 100.)?)?;
    let middle = subtract(&middle, &beyond([30., 30., 0.], [0., 1., 1.], 100.)?)?;
    let last = subtract(&rod([30., 30., -10.], [30., 30., 30.], 2.)?, &beyond([30., 30., 0.], [0., -1., -1.], 100.)?)?;
    union(&union(&first, &middle)?, &last)
}
