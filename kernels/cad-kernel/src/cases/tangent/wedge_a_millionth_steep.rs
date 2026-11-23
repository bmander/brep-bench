//! A cube of side 10 from (−5, −5, −10) to (5, 5, 0) in common with the half-space above the plane
//! z = y·10⁻⁶: a wedge 5·10⁻⁶ thick at y = −5, thinning to an edge along the x axis.
//!
//! The half-space is a block standing on the plane, reaching past the cube on every other side,
//! taken in common with the cube. (cad-kernel's `build::kept`, the same common with the cube as the
//! first operand, is refused in 0.21.0: "edge 12 near [5.0, −0.00022, 0.0]: faces 8 and 15 (a
//! plane and a plane) coincide along it"; with the half-space first the complex orders the faces.)
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, square};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const HALF: f64 = 5.;
const SLOPE: f64 = 1e-6;
/// How far the half-space's block reaches along the plane each way, and above it.
const REACH: f64 = 20.;

pub fn build() -> Result<Brep, String> {
    let cube = block([-HALF, -HALF, -2. * HALF], [HALF, HALF, 0.])?;
    // the side of the plane through the x axis its normal (0, −SLOPE, 1) points to
    let above = prism(&square([0.; 3], unit([0., -SLOPE, 1.]), REACH), 0., REACH)?;
    Ok(combined(&above, &cube, Op::Common, 0.)?)
}
