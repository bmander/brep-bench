//! A cube of side 10 at the origin, a slot a millionth wide (x from 5 to 5.000001) sawn down into it
//! from the top to z = 5, right through it in y.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const SLOT_AT: f64 = 5.;
const SLOT_WIDTH: f64 = 1e-6;
const SLOT_FLOOR: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // the saw's block runs 1 past the cube's front, back and top
    let saw = block([SLOT_AT, -1., SLOT_FLOOR], [SLOT_AT + SLOT_WIDTH, SIDE + 1., SIDE + 1.])?;
    Ok(combined(&cube, &saw, Op::Cut, 0.)?)
}
