//! A block on the square 0 ≤ x, y ≤ 20, floor at z = 0, under a clamped bicubic B-spline
//! surface of 6 × 5 control points with a double knot in u.
use cad_kernel::build::lofted;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

const SIDE: f64 = 20.;
const XS: [f64; 6] = [0., 3., 7., 11., 16., 20.];
const YS: [f64; 5] = [0., 5., 10., 15., 20.];
const HEIGHTS: [[f64; 5]; 6] = [
    [3., 5., 6., 5., 4.],
    [5., 9., 11., 8., 5.],
    [6., 12., 15., 10., 6.],
    [4., 10., 18., 9., 5.],
    [5., 8., 9., 7., 4.],
    [2., 4., 5., 4., 3.],
];
const U_KNOTS: [f64; 10] = [0., 0., 0., 0., 0.4, 0.4, 1., 1., 1., 1.];
const V_KNOTS: [f64; 9] = [0., 0., 0., 0., 1. / 3., 1., 1., 1., 1.];

/// A net running round the block's section (`u`) and along x (`v`, as the surface's u runs, on
/// its knots): `rows[k]` the poles of its kth curve over x.
fn net(du: usize, uknots: Vec<f64>, rows: Vec<Vec<[f64; 3]>>) -> Net {
    Net { du, dv: 3, uknots, vknots: U_KNOTS.to_vec(), poles: rows, weights: None }
}

pub fn build() -> Result<Brep, String> {
    let floor = |y: f64| XS.iter().map(|&x| [x, y, 0.]).collect::<Vec<_>>();
    let edge = |j: usize| (0..XS.len()).map(|i| [XS[i], YS[j], HEIGHTS[i][j]]).collect::<Vec<_>>();
    let linear = vec![0., 0., 1., 1.];
    // the top walked from y = 20 back to y = 0: the v curve reversed, its knots reflected
    let back: Vec<f64> = V_KNOTS.iter().rev().map(|k| 1. - k).collect();
    let over = (0..YS.len()).rev().map(edge).collect();
    // the section in the plane x = const, counter-clockwise about +x: floor, the side y = 20,
    // the top, the side y = 0
    let nets = vec![
        net(1, linear.clone(), vec![floor(0.), floor(SIDE)]),
        net(1, linear.clone(), vec![floor(SIDE), edge(YS.len() - 1)]),
        net(3, back, over),
        net(1, linear, vec![edge(0), floor(0.)]),
    ];
    lofted([0., 0., 0.], [1., 0., 0.], [0., SIDE], vec![nets])
}
