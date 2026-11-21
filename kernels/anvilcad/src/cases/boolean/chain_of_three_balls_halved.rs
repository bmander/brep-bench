//! The chain of three balls (radius 5, 4 and 3 about the origin, (7, 0, 0) and (10, 4, 0)) cut by
//! the plane z = 0, keeping z ≥ 0.
use anvil_brep::Solid;

use crate::shared::{ball, block, subtract, union};

pub fn build() -> Result<Solid, String> {
    let ab = union(&ball([0., 0., 0.], 5.)?, &ball([7., 0., 0.], 4.)?)?;
    let chain = union(&ab, &ball([10., 4., 0.], 3.)?)?;
    // everything below z = 0, and more than the chain's reach
    let below = block([-10., -10., -10.], [20., 20., 0.])?;
    subtract(&chain, &below)
}
