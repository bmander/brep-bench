//! The elliptic cylinder of `geometry/elliptic-cylinder` (x²/25 + y²/9 ≤ 1 from z = 0 to 10)
//! shrunk by 2.5, past its least radius of curvature b²/a = 1.8: its side's offset swallowtails
//! near the ends of the long axis, and the solid of every point at least 2.5 inside keeps only its
//! part clear of them, meeting itself in a sharp upright edge at each end.
//!
//! Refused, by the kernel: "face 0 (extrusion) is not swept along its normal: a extrusion has no
//! offset in closed form". cad-kernel has no ellipse in a profile, so the side is the rational
//! B-spline ellipse extruded (`Surface::Extrusion`), and `offset::shrunk` sweeps a face along its
//! normal only on a plane, cylinder, cone, sphere or torus (`offset::normal_sweep`); an ellipse's offset
//! is no conic. Nothing else in 0.21.0 moves the side: `Surface::parallel` reads only a B-spline
//! sheet's offset, not an extrusion's; `build::offset` refuses a spline in a profile; and the
//! circular cylinder stretched into an elliptic one is an affine image (`Surface::Affine`), which
//! has no offset either (`Surface::has_offset`).
use crate::cases::geometry::elliptic_cylinder;
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    shrunk(&elliptic_cylinder::build()?, 2.5, Join::Round)
}
