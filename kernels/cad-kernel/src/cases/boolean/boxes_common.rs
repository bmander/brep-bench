//! The part common to two blocks 10 high, over x, y from 0 to 10 and from 5 to 15.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const HEIGHT: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let first = block([0., 0., 0.], [10., 10., HEIGHT])?;
    let second = block([5., 5., 0.], [15., 15., HEIGHT])?;
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
