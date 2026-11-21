//! A cylinder of radius 10 on the z axis from z = 0 to 20, its side parted at z = 10 and each half
//! drafted 3° away from the parting line: the upper pulling along +z, the lower along −z.
use anvil_brep::{Frame, Solid};

use crate::shared::{cylinder, draft, ring, union};

pub fn build() -> Result<Solid, String> {
    // the side split at the parting line: the two halves, each drafted about it, joined there
    let w = Frame::world();
    let upper = draft(&cylinder(&w, 10., 10., 20.)?, &[ring(10., 15.)], [0., 0., 10.], [0., 0., 1.], 3.)?;
    let lower = draft(&cylinder(&w, 10., 0., 10.)?, &[ring(10., 5.)], [0., 0., 10.], [0., 0., -1.], 3.)?;
    union(&upper, &lower)
}
