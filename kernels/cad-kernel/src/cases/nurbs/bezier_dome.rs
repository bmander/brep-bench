//! A block on the square 0 ≤ x, y ≤ 20, floor at z = 0, under the bicubic Bézier patch whose
//! control points stand over the even 4 × 4 grid of the square at the heights `HEIGHTS`.
use cad_kernel::build::lofted;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

pub const SIDE: f64 = 20.;
/// Row i at x = 20i/3, column j at y = 20j/3.
pub const HEIGHTS: [[f64; 4]; 4] = [
    [4., 6., 7., 5.],
    [6., 14., 12., 7.],
    [5., 13., 16., 8.],
    [3., 6., 8., 6.],
];

/// A net running round the block's section (`u`) and along x (`v`, cubic, as the patch runs):
/// row k of poles is `row(k)`, the curve over x at that step round.
fn net(du: usize, rows: impl Fn(usize) -> [[f64; 3]; 4]) -> Net {
    let clamped = |d: usize| [vec![0.; d + 1], vec![1.; d + 1]].concat();
    Net { du, dv: 3, uknots: clamped(du), vknots: clamped(3),
          poles: (0..=du).map(|k| rows(k).to_vec()).collect(), weights: None }
}

pub fn build() -> Result<Brep, String> {
    let x = |i: usize| SIDE * i as f64 / 3.;
    let floor = |y: f64| move |i: usize| [x(i), y, 0.];
    let edge = |j: usize| move |i: usize| [x(i), SIDE * j as f64 / 3., HEIGHTS[i][j]];
    let row = |f: &dyn Fn(usize) -> [f64; 3]| std::array::from_fn(f);
    // the section in the plane x = const, walked counter-clockwise about +x: along the floor to
    // y = 20, up that side, back over the top to y = 0 and down the side there
    let nets = vec![
        net(1, |k| if k == 0 { row(&floor(0.)) } else { row(&floor(SIDE)) }),
        net(1, |k| if k == 0 { row(&floor(SIDE)) } else { row(&edge(3)) }),
        net(3, |k| row(&edge(3 - k))),
        net(1, |k| if k == 0 { row(&edge(0)) } else { row(&floor(0.)) }),
    ];
    // lofted along x from the plane x = 0 to x = 20, capped by them
    lofted([0., 0., 0.], [1., 0., 0.], [0., SIDE], vec![nets])
}
