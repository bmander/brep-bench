//! A sheet of two flat faces meeting at 60° along a crease, the floor (the rectangle in the xy
//! plane from (0, 0, 0) to (10, 8, 0)) and the wall (the rectangle through (0, 0, 0), (0, 8, 0),
//! (5, 8, 5√3) and (5, 0, 5√3), leaning over the floor), thickened by 1 to the outside of the
//! crease (−z from the floor, away from the floor from the wall), the moved faces extended until
//! they meet, so the corner stays sharp.
//!
//! The sheet is the open polyline (10, 0), (0, 0), (5, 5√3) on the Front plane surface-extruded 8
//! along +y, as `thicken/angle-bracket-sharp` makes its L; cadrs's Thicken of it, 1 toward −z from
//! the floor (and so away from the floor from the wall), its bend mitred (`sharp`).
use crate::bench::{p3, v3};
use crate::shared::thicken::{open_chain, polyline, surface_extrude, thicken_toward_with};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Plane};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let top = 5.0 * 3f64.sqrt();
    let s = open_chain(Plane::xz(0.0), polyline(&[(10.0, 0.0), (0.0, 0.0), (5.0, top)]));
    let sheet = surface_extrude(k, &s, v3(0.0, 1.0, 0.0), 8.0)?;
    thicken_toward_with(k, sheet, p3(5.0, 4.0, 0.0), v3(0.0, 0.0, -1.0), 1.0, true)
}
