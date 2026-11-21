//! A cube of side 10 with all twelve edges rounded to radius 1.
use anvil_brep::Solid;

use crate::shared::{block, fillet};

pub fn build() -> Result<Solid, String> {
    let cube = block([0., 0., 0.], [10., 10., 10.])?;
    // every edge, picked by its middle
    let mut edges = Vec::new();
    for a in [0., 10.] {
        for b in [0., 10.] {
            edges.extend([[5., a, b], [a, 5., b], [a, b, 5.]]);
        }
    }
    fillet(&cube, &edges, 1.)
}
