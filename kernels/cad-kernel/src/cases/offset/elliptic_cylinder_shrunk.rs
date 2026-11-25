//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to 10)
//! shrunk by 1: every point of it at least 1 inside it.
//!
//! The side is the rational B-spline ellipse extruded (`Surface::Extrusion`), whose offset has no
//! closed form: `offset::shrunk` sweeps it as the face family of the rectangle each line of it
//! sweeps along its normal (`offset::extruded_sweep`), closed round the ellipse.
use crate::cases::geometry::elliptic_cylinder;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder::build()?, 1., Join::Round)
}
