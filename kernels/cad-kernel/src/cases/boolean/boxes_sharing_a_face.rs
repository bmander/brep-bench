//! Two cubes of side 10 joined face to face, the face they share gone: (0..20, 0..10, 0..10).
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;

pub fn build() -> Result<Brep, String> {
    let first = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    let second = block([SIDE, 0., 0.], [2. * SIDE, SIDE, SIDE])?;
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
