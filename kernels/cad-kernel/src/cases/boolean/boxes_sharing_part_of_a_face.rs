//! A cube of side 10 at the origin, with one from (5, 0, 10) to (15, 10, 20) standing on it.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let first = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    let second = block([5., 0., SIDE], [5. + SIDE, SIDE, 2. * SIDE])?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
