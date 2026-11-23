//! A prism from z = 0 to z = 6 on a closed spline, the periodic uniform cubic B-spline on the eight
//! control points (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4) — a bean
//! dented at the top — the spline where the prism's side meets its top rounded to radius 1: the
//! pipe a ball of radius 1 sweeps rolling round the inside of the rim touching the top and the
//! side, convex round the bean's ends and saddle-shaped through the dent.
//!
//! The side is lofted as one closed cubic B-spline sheet, the spline carried straight up, and the
//! rim rounded half by half as `elliptic_cylinder_rim` rounds its rim (the module says why), the
//! bean halved by the plane x = 0 it is symmetric about, square to the rim where it crosses.
use super::elliptic_cylinder_rim::rounded_by_halves;
use cad_kernel::build::lofted;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

const POLES: [[f64; 2]; 8] = [
    [-10., -5.], [0., -7.], [10., -5.], [12., 4.], [4., 7.], [0., 3.], [-4., 7.], [-12., 4.],
];
const HEIGHT: f64 = 6.;
const ROUND: f64 = 1.;
/// What each round's canal is fitted within (mm): at the part's own tolerance (2.4e-8 mm, the bar
/// 2.4e-6 mm) the densest net misses the saddle through the dent by 2.8e-6 mm, and the ball's
/// traced centre is read not to lie between its run's ends.
const FIT: f64 = 1e-5;

/// The prism on the spline, lofted up z on one closed sheet: the periodic spline as a B-spline on
/// eleven poles (the eight counted round from `first`, the first three again after them) over the
/// knots 0, 1, …, 14, carried straight up. Its seam stands where the segment on the poles `first`
/// to `first` + 3 starts.
fn prism(first: usize) -> Result<Brep, String> {
    let side = Net {
        du: 3,
        dv: 1,
        uknots: (0..15).map(f64::from).collect(),
        vknots: vec![0., 0., 1., 1.],
        poles: (0..11).map(|k| POLES[(first + k) % 8])
            .map(|[x, y]| vec![[x, y, 0.], [x, y, HEIGHT]]).collect(),
        weights: None,
    };
    lofted([0., 0., 0.], [0., 0., 1.], [0., HEIGHT], vec![vec![side]])
}

/// The point the spline's segment on the poles `i` to `i` + 3 starts at, on the rim.
fn on_rim(i: usize) -> [f64; 3] {
    let [a, b, c] = [0, 1, 2].map(|k| POLES[(i + k) % 8]);
    [(a[0] + 4. * b[0] + c[0]) / 6., (a[1] + 4. * b[1] + c[1]) / 6., HEIGHT]
}

pub fn build() -> Result<Brep, String> {
    // the half x ≥ 0 on the sheet seamed at the far end of the other, (−10.33, 3), its rim picked
    // at (8.67, −3.83); the half x ≤ 0 likewise mirrored
    let part = |s: f64| prism(if s > 0. { 6 } else { 2 });
    rounded_by_halves(&part, [1., 0., 0.], &|s| on_rim(if s > 0. { 1 } else { 7 }), ROUND, FIT)
}
