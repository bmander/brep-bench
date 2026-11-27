//! The plate from the origin to (40, 30, 6) with all twelve edges rounded to radius 3, half its
//! thickness, in one operation.
use anvil_brep::Solid;

use crate::shared::{block, fillet};

pub fn build() -> Result<Solid, String> {
    let (x, y, z) = (40., 30., 6.);
    let plate = block([0., 0., 0.], [x, y, z])?;
    let mut edges = Vec::new();
    for b in [0., z] {
        for a in [0., y] {
            edges.push([x / 2., a, b]);
        }
        for a in [0., x] {
            edges.push([a, y / 2., b]);
        }
    }
    for a in [0., x] {
        for b in [0., y] {
            edges.push([a, b, z / 2.]);
        }
    }
    fillet(&plate, &edges, 3.)
}
