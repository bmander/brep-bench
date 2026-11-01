//! A sheet: the parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, made as a B-spline
//! surface: of degree 2 in u and 1 in v, with no interior knots (u and v each from 0 to 1, their
//! end knots of full multiplicity), all weights 1, and control points (−4, y, 2), (0, y, −2) and
//! (4, y, 2) in u, for y = 0 and y = 10 in v. Its quadratic Bézier is exactly the parabola, with x
//! running evenly in u. It is one face, bounded by two parabolic arcs, at y = 0 and y = 10, and two
//! straight lines, at x = −4 and x = 4.
//!
//! Refused. cad-kernel has no thickening of a sheet: it makes sheets (`build::sheet`,
//! `build::sheets`) and cuts solids with them, but it has no offset of a face or a shell, and so no
//! solid between a sheet and its offset (its README lists "shelling and offsets" among what comes
//! next).
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("no thickening of a sheet: cad-kernel has no offset of a face or a shell".into())
}
