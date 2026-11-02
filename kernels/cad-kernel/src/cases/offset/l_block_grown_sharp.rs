//! The L-shaped plate through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15), 8 thick, grown
//! by 1, its edges kept sharp: its faces moved out and extended until they meet.
use crate::sketch::l_block;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    grown(&l_block()?, 1., Join::Sharp)
}
