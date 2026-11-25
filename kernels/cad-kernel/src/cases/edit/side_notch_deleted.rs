//! The box from the origin to (20, 10, 10) with a half-round notch in its side y = 0, the cylinder
//! of radius 3 about the line through (10, 0) cut away top to bottom; the notch's half cylinder
//! then deleted, the side running on from x = 7 and from x = 13 until it is one rectangle again,
//! and the top and the bottom closing over the half disks the notch took.
//!
//! The notched block is the box, a `prism`, cut by a `prism` of a `disc` standing past its top and
//! bottom; the half cylinder is picked by a point on it and deleted (`defeature::deleted`).
use crate::sketch::{block, deleted_through};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let cutter = prism(&disc([10., 0., -1.], [0., 0., 1.], 3.), 0., 12.)?;
    let notched = combined(&block([0., 0., 0.], [20., 10., 10.])?, &cutter, Op::Cut, 0.)?;
    deleted_through(&notched, &[[10., 3., 5.]])
}
