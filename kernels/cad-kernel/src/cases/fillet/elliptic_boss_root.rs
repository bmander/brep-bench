//! A plate from (−20, −15, 0) to (20, 15, 4) joined to an elliptic boss on its top, the elliptic
//! cylinder on x²/10² + y²/6² = 1 (semi-axes 10 along x and 6 along y) from z = 4 to z = 10, the
//! boss's root, where its side meets the plate's top, rounded to radius 2: the pipe a ball of
//! radius 2 sweeps rolling round the boss in the corner, filled in. The boss's top rim stays sharp.
//!
//! The boss's side is a closed rational B-spline sheet, and the root is rounded half by half as
//! `elliptic_cylinder_rim` rounds its rim (the module says why).
use super::elliptic_cylinder_rim::{cylinder, rounded_by_halves};
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const HALF_LENGTH: f64 = 20.;
const HALF_WIDTH: f64 = 15.;
const THICKNESS: f64 = 4.;
const A: f64 = 10.;
const B: f64 = 6.;
const BOSS_HEIGHT: f64 = 6.;
const ROUND: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let part = |s: f64| -> Result<Brep, String> {
        let plate = block([-HALF_LENGTH, -HALF_WIDTH, 0.], [HALF_LENGTH, HALF_WIDTH, THICKNESS])?;
        let boss = cylinder(A, B, [THICKNESS, THICKNESS + BOSS_HEIGHT], s)?;
        Ok(combined(&plate, &boss, Op::Union, 0.)?)
    };
    // halved across y; each half's root picked by its point on the minor axis
    rounded_by_halves(&part, [0., 1., 0.], &|s| [0., s * B, THICKNESS], ROUND, 0.)
}
