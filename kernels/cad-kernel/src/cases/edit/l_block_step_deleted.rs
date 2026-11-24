//! The L-shaped block through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15), 8 high, its
//! step, the riser x = 5 and the tread y = 5, then deleted together: the end y = 15 runs on to
//! x = 20 and the side x = 20 on to y = 15, meeting in a new edge along x = 20, y = 15, and the top
//! and the bottom close in over the corner.
//!
//! The edit is the kernel's: `defeature::deleted` runs the faces round the deleted ones on along
//! their own surfaces and closes the gap they part off.
use crate::sketch::{deleting, faces_through, l_block};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let b = l_block()?;
    // the riser x = 5 and the tread y = 5
    let step = faces_through(&b, &[[5., 10., 4.], [12., 5., 4.]])?;
    deleting(&b, &step)
}
