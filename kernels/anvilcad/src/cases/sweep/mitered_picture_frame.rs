//! A picture frame: the molding, 8 wide and 6 tall with a rabbet 3 wide and 2 deep under its inner
//! edge, swept round the rectangle with corners (±30, ±20, 0), mitered at its corners.
use anvil_brep::Solid;

use crate::shared::{block, subtract};

pub fn build() -> Result<Solid, String> {
    // anvil-brep's sweep turns no sharp corner. Each side's section swept straight and mitered on
    // its corners' diagonals is, all four together, the ring between the rectangle grown by the
    // section's width and the path's rectangle, less the rabbet: the rectangle grown by 3 up to
    // z = 2
    let outer = block([-38., -28., 0.], [38., 28., 6.])?;
    let opening = subtract(&outer, &block([-30., -20., -1.], [30., 20., 7.])?)?;
    subtract(&opening, &block([-33., -23., -1.], [33., 23., 2.])?)
}
