//! A box from the origin to (20, 10, 6), the four edges round its top chamfered 1 across the top
//! and 2 down the sides.
use anvil_brep::Solid;

use crate::shared::{block, chamfer_angle};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no chamfer of two distances: on faces square to each other, 1 across the top
    // and 2 down the side is 1 along the top leaving it at atan(2/1)
    let angle = 2f64.atan().to_degrees();
    let mut b = block([0., 0., 0.], [20., 10., 6.])?;
    for (edge, along) in [
        ([10., 0., 6.], [10., 5., 6.]),
        ([20., 5., 6.], [10., 5., 6.]),
        ([10., 10., 6.], [10., 5., 6.]),
        ([0., 5., 6.], [10., 5., 6.]),
    ] {
        b = chamfer_angle(&b, edge, along, 1., angle)?;
    }
    Ok(b)
}
