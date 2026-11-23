//! A plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
//! x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y), the hole's rim on the top face rounded
//! to radius 1: the pipe a ball of radius 1 sweeps rolling round the rim touching the top and the
//! hole's wall. The rim on the bottom face stays sharp.
//!
//! The hole is cut by an elliptic cylinder whose side is a closed rational B-spline sheet, and the
//! rim is rounded half by half as `elliptic_cylinder_rim` rounds its rim (the module says why).
use super::elliptic_cylinder_rim::{cylinder, rounded_by_halves};
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const HALF_LENGTH: f64 = 15.;
const HALF_WIDTH: f64 = 10.;
const THICKNESS: f64 = 4.;
const A: f64 = 6.;
const B: f64 = 3.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let plate = |s: f64| -> Result<Brep, String> {
        let plate = block([-HALF_LENGTH, -HALF_WIDTH, 0.], [HALF_LENGTH, HALF_WIDTH, THICKNESS])?;
        // the drill runs past both faces, from z = −1 to z = 5
        let drill = cylinder(A, B, [-1., THICKNESS + 1.], s)?;
        Ok(combined(&plate, &drill, Op::Cut, 0.)?)
    };
    // halved across y; each half's top rim picked by its point on the minor axis
    rounded_by_halves(&plate, [0., 1., 0.], &|s| [0., s * B, THICKNESS], ROUND, 0.)
}
