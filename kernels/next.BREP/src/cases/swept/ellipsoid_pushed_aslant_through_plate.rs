//! The plate from (−15, −15, 0) to (15, 15, 4), less everything the ellipsoid of semi-axes 3, 2
//! and 1.5 centred at (−2, −1.5, −4) passes through as it slides without turning along (4, 3, 12).
//!
//! The kernel's whole sweep of an ellipsoid slid along a line (`sweep_ellipsoid_along_line`), cut
//! from the plate.
use crate::solids::p;
use crate::solids::swept::less_all;
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_ellipsoid_along_line, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 4.0)?;
    let (x, y) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0));
    let swept =
        sweep_ellipsoid_along_line(p(-2.0, -1.5, -4.0), x, y, [3.0, 2.0, 1.5], p(4.0, 3.0, 12.0))?;
    less_all(plate, vec![swept])
}
