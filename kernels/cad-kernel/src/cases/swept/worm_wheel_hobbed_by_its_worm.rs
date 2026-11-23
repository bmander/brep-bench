//! A worm wheel, the disc of radius 32 about the z axis from z = −6 to 6, hobbed whole by its
//! single-start worm (lead 2π, pitch radius 10) on the line through (0, 40, 0) along x, from x = −20
//! to 20: the worm turns by t about its axis while the wheel turns by t/30, t from 0 to 60π.
//!
//! The thread is its axial trapezoid screwed (`build::screwed`, exact), cut to its ends and placed.
//!
//! Refused by the kernel at run time. Seen from the wheel the worm's turn is no constant twist, and
//! the kernel's sweep cut from a blank (`swept::closed::cut_from`) reads a tool under such a motion
//! only as a convex solid of revolution (K2) or as all planes and lines (`swept::general`); the
//! thread's helicoidal flanks are neither, and it is refused by what it is (`Unbuilt::Curved`).
use crate::gears::hobbed;
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let wheel = prism(&disc([0., 0., -6.], [0., 0., 1.], 32.), 0., 12.)?;
    hobbed(&wheel, [0., 40., 0.], -20., 20.)
}
