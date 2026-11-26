//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick, its
//! inside corner's edge along (5, 5) rounded to radius 4, grown by 5, more than the round's radius.
use crate::sketch::{l_block, rolled};
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the concave upright edge, picked by its middle, filled with the quarter cylinder about the
    // line x = 9, y = 9
    let rounded = rolled(&l_block()?, &[[5., 5., 4.]], 4.)?;
    grown(&rounded, 5., Join::Round)
}
