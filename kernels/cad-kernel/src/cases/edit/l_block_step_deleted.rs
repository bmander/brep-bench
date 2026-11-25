//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! step, the riser x = 5 and the tread y = 5, then deleted together: the end y = 15 runs on to
//! x = 20 and the side x = 20 on to y = 15, meeting in a new edge along x = 20, y = 15, and the top
//! and the bottom close in over the corner.
//!
//! The L-block is `sketch::l_block`, its outline swept up by `prism`; the riser and the tread are
//! picked by a point on each and deleted together (`defeature::deleted`).
use crate::sketch::{deleted_through, l_block};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    deleted_through(&l_block()?, &[[5., 10., 4.], [12., 5., 4.]])
}
