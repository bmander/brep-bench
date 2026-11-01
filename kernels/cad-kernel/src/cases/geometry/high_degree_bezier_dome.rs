//! A block on the square 0 ≤ x, y ≤ 20, floor at z = 0, under the Bézier patch of degree 15 each
//! way whose control points stand over the even 16 × 16 grid of the square at the heights
//! 4 + ((3i + 5j) mod 7).
use cad_kernel::build::lofted;
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

const SIDE: f64 = 20.;
const DEGREE: usize = 15;

fn height(i: usize, j: usize) -> f64 { (4 + (3 * i + 5 * j) % 7) as f64 }

/// A net running round the block's section (`u`, of degree `du`) and along x (`v`, of degree 15,
/// as the patch runs): row k of poles is `rows(k)`, the curve over x at that step round.
fn net(du: usize, rows: impl Fn(usize) -> Vec<[f64; 3]>) -> Net {
    let clamped = |d: usize| [vec![0.; d + 1], vec![1.; d + 1]].concat();
    Net { du, dv: DEGREE, uknots: clamped(du), vknots: clamped(DEGREE),
          poles: (0..=du).map(rows).collect(), weights: None }
}

pub fn build() -> Result<Brep, String> {
    let at = |k: usize| SIDE * k as f64 / DEGREE as f64;
    let floor = |y: f64| (0..=DEGREE).map(|i| [at(i), y, 0.]).collect::<Vec<_>>();
    let edge = |j: usize| (0..=DEGREE).map(|i| [at(i), at(j), height(i, j)]).collect::<Vec<_>>();
    // the section in the plane x = const, walked counter-clockwise about +x: along the floor to
    // y = 20, up that side, back over the top to y = 0 and down the side there
    let nets = vec![
        net(1, |k| if k == 0 { floor(0.) } else { floor(SIDE) }),
        net(1, |k| if k == 0 { floor(SIDE) } else { edge(DEGREE) }),
        net(DEGREE, |k| edge(DEGREE - k)),
        net(1, |k| if k == 0 { edge(0) } else { floor(0.) }),
    ];
    // lofted along x from the plane x = 0 to x = 20, capped by them
    lofted([0., 0., 0.], [1., 0., 0.], [0., SIDE], vec![nets])
}
