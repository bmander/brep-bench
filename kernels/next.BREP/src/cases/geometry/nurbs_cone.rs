//! A cone on the disc of radius 5 about the z axis in the plane z = 0, its apex at (0, 0, 10), its
//! side one NURBS face: the 9 × 2 net whose first row is the base circle as a rational quadratic
//! and whose second is nine copies of the apex with the same weights, so that its edge v = 1 is a
//! pole. The side and the flat disc under it bound the solid; no cone is used.
use crate::solids::geometry::{uv_line, Records};
use crate::solids::p;
use brep_kernel::{make_plane, BrepSolid, NurbsSurface, Vec4};
use std::f64::consts::FRAC_1_SQRT_2 as H;

pub fn build() -> Result<BrepSolid, String> {
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

    let mut b = Records::new();
    let rim_start = b.vertex(side.evaluate(0.0, 0.0)?);
    let top = b.vertex(side.evaluate(0.0, 1.0)?);
    let rim = b.edge(side.iso_curve_v(0.0)?, rim_start, rim_start)?;
    let seam = b.edge(side.iso_curve_u(0.0)?, rim_start, top)?;
    let pole = b.pole(top)?;
    b.face(side, vec![
        (rim, true, uv_line(0.0, 0.0, 1.0, 0.0)?),
        (seam, true, uv_line(1.0, 0.0, 1.0, 1.0)?),
        (pole, true, uv_line(1.0, 1.0, 0.0, 1.0)?),
        (seam, false, uv_line(0.0, 1.0, 0.0, 0.0)?),
    ]);
    // the base, its plane's normal (y × x) down and out, bounded by the rim run backwards
    let base = make_plane(p(-5.0, -5.0, 0.0), p(0.0, 1.0, 0.0), p(1.0, 0.0, 0.0), 10.0, 10.0)?;
    b.flat(base, &[(rim, false)])?;
    b.solid(0)
}
