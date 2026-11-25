//! The ball of radius 5 about the origin with its top cut flat, everything above z = 3 cut away,
//! leaving a flat disc of radius 4 at z = 3; the flat then deleted, the ball closing over the
//! circle of radius 4 its edge is.
//!
//! The flattened ball is `build::sphere` `kept` below the plane z = 3; the flat is picked by a
//! point on it and deleted (`defeature::deleted`).
use crate::sketch::deleted_through;
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let flattened = kept(&sphere([0., 0., 0.], [0., 0., 1.], 5.)?, [0., 0., 3.], [0., 0., -1.], 0.)?;
    deleted_through(&flattened, &[[1., 0., 3.]])
}
