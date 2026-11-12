//! A cone on the disc of radius 5 about the z axis in the plane z = 0, its apex at (0, 0, 10), its
//! side one NURBS face: the 9 × 2 net whose first row is the base circle as a rational quadratic
//! and whose second is nine copies of the apex with the same weights, so that its edge v = 1 is a
//! pole. The side is the kernel's face on that surface (`face_from_surface`, which finds its seam
//! and its pole), the base its flat face inside the rim, and the two are joined into the solid
//! they bound (`solid_from_sheets`); no cone is used.
use crate::solids::nurbs::disc;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{face_from_surface, solid_from_sheets, BrepSolid, NurbsSurface, Vec4};
use std::f64::consts::FRAC_1_SQRT_2 as H;

pub fn build() -> Result<BrepSolid, Refusal> {
    let circle = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0), (-1.0, 1.0), (-1.0, 0.0), (-1.0, -1.0),
                  (0.0, -1.0), (1.0, -1.0), (1.0, 0.0)];
    let circle_w = [1.0, H, 1.0, H, 1.0, H, 1.0, H, 1.0];
    let apex = p(0.0, 0.0, 10.0);
    let net = circle
        .iter()
        .zip(circle_w)
        .map(|(&(cx, cy), w)| {
            vec![Vec4::from_point(p(5.0 * cx, 5.0 * cy, 0.0), w), Vec4::from_point(apex, w)]
        })
        .collect();
    let round = vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0];
    let side = NurbsSurface::new(2, 1, round, vec![0.0, 0.0, 1.0, 1.0], net)?;

    // the side, which closes on its seam and at its apex, and the base inside its rim
    let base = disc(5.0, 0.0, side.iso_curve_v(0.0)?)?;
    Ok(solid_from_sheets(&[face_from_surface(&side)?, base], 1e-7)?)
}
