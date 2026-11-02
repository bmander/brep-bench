//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide, 6 high, shelled
//! inward to a wall 1.5 thick, open at its top.
use crate::sketch::{polygon, faces_through};
use cad_kernel::build::prism;
use cad_kernel::offset::{shelled, Join, Side};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let body = prism(&polygon(&[[0., 0., 0.], [10., 0., 0.], [10., 4., 0.], [14., 4., 0.], [14., 0., 0.], [24., 0., 0.], [24., 10., 0.], [14., 10., 0.], [14., 6., 0.], [10., 6., 0.], [10., 10., 0.], [0., 10., 0.]]), 0., 6.)?;
    let open = faces_through(&body, &[[2., 2., 6.]])?;
    shelled(&body, &open, 1.5, Side::In, Join::Round)
}
