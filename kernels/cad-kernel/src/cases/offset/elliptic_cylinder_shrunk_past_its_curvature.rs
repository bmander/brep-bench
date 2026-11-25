//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to 10)
//! shrunk by 2.5, past its least radius of curvature b²/a = 1.8: its side's offset swallowtails
//! near the ends of the long axis, and the solid of every point at least 2.5 inside keeps only its
//! part clear of them, meeting itself in a sharp upright edge at each end.
//!
//! Refused, by the kernel: "face 0 (extrusion) is not swept along its normal: its sweep to -2.5
//! folds where its curve turns tighter than that". `offset::shrunk` sweeps the extruded side as
//! the face family of the rectangle each line of it sweeps along its normal
//! (`offset::extruded_sweep`), which past the ellipse's least radius of curvature would fold; the
//! part of the sweep past where it meets itself, which no point nearest the side stands in, is not
//! cut away yet.
use crate::cases::geometry::elliptic_cylinder;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder::build()?, 2.5, Join::Round)
}
