//! A sheet: the parabolic cylinder z = x²/8 for −4 ≤ x ≤ 4 and 0 ≤ y ≤ 10, made as a B-spline
//! surface: of degree 2 in u and 1 in v, with no interior knots (u and v each from 0 to 1, their
//! end knots of full multiplicity), all weights 1, and control points (−4, y, 2), (0, y, −2) and
//! (4, y, 2) in u, for y = 0 and y = 10 in v. Its quadratic Bézier is exactly the parabola, with x
//! running evenly in u. It is one face, bounded by two parabolic arcs, at y = 0 and y = 10, and two
//! straight lines, at x = −4 and x = 4.
//!
//! Refused, by the kernel: a B-spline surface has no offset in closed form, so `thickened` has no
//! exact face to move the sheet to (its offset is no B-spline of any degree), and says so.
use cad_kernel::build::sheet;
use cad_kernel::nurbs::Net;
use cad_kernel::offset::{thickened, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let row = |y: f64| vec![[-4., y, 2.], [0., y, -2.], [4., y, 2.]];
    let (a, b) = (row(0.), row(10.));
    let net = Net { du: 2, dv: 1, uknots: vec![0., 0., 0., 1., 1., 1.], vknots: vec![0., 0., 1., 1.],
                    poles: (0..3).map(|i| vec![a[i], b[i]]).collect(), weights: None };
    let trough = sheet(net)?;
    // its concave side, toward the focus: the side its normal points to, or the other
    let up = trough.faces[0].surface.normal([0.5, 0.5]).map_or(false, |n| n[2] > 0.) != trough.faces[0].reversed;
    thickened(&trough, if up { [0., 1.] } else { [-1., 0.] }, Join::Round)
}
