//! A face gear, the ring between radii 40 and 52 about the z axis from z = −6 to 2, hobbed whole by
//! a single-start worm (lead 2π, pitch radius 10) on the line through (0, 30, 10) along x, from
//! x = 0 to 52: the worm turns by t about its axis while the gear turns by t/30, t from 0 to 60π.
//!
//! The ring is a disc less a disc, each a prism; the thread its axial trapezoid screwed
//! (`build::screwed`, exact), cut to its ends and placed. Seen from the gear the worm's turn is no
//! constant twist, and the kernel's arrangement cuts the sweep from the blank
//! (`swept::closed::cut_from`) under such a motion only for a convex solid of revolution; it refuses
//! the thread by what it is.
use crate::gears::hobbed;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let outer = prism(&disc([0., 0., -6.], up, 52.), 0., 8.)?;
    let bore = prism(&disc([0., 0., -7.], up, 40.), 0., 10.)?;
    let ring = combined(&outer, &bore, Op::Cut, 0.)?;
    hobbed(&ring, [0., 30., 10.], 0., 52.)
}
