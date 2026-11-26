//! The solid the box from (0, 0, 0) to (10, 6, 4) fills as it slides without turning along
//! (8, 5, 0), level, to the box from (8, 5, 0) to (18, 11, 4): its top and bottom slide in their
//! own planes, so it is the prism 4 high over the hexagon (0, 0), (10, 0), (18, 5), (18, 11),
//! (8, 11), (0, 6).
//!
//! The box is a rectangle on the Top plane extruded 4; its slide is cadrs's sweep of a solid
//! (`Kernel::sweep_solid`, one slide along (8, 5, 0)), as in `swept/box-slid-aslant`.
use crate::bench::{block, p3, v3};
use crate::shared::swept::slid;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let start = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 6.0, 4.0))?;
    slid(k, start, v3(8.0, 5.0, 0.0))
}
