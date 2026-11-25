//! The box from the origin to (20, 10, 10), the edge where its top, z = 10, meets its side x = 20
//! beveled 3 back on each face, the bevel the plane through the lines x = 17, z = 10 and x = 20,
//! z = 7; the bevel then deleted, the top running on from x = 17 and the side from z = 7 until they
//! meet in the edge x = 20, z = 10.
//!
//! The beveled block is its section, the pentagon in y = 0, swept 10 along y by `prism`; the bevel
//! is picked by a point on it and deleted (`defeature::deleted`).
use crate::sketch::{deleted_through, polygon};
use cad_kernel::build::prism;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // counter-clockwise in (x, z), so its normal is −y: ordinates −10 to 0 along it are y 10 to 0
    let section = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 0., 7.], [17., 0., 10.], [0., 0., 10.]]);
    let beveled = prism(&section, -10., 0.)?;
    deleted_through(&beveled, &[[18.5, 5., 8.5]])
}
