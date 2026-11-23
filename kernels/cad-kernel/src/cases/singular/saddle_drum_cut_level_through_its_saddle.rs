//! The drum of geometry/saddle-drum (radius 8, floor z = 0, top the saddle z = 10 + xy/8 made as
//! the bilinear B-spline patch), cut level at z = 10, the plane tangent to the saddle at its saddle
//! point.
//!
//! The block under the patch is cut level first and then trimmed by the drum's cylinder: the same
//! set as the drum cut level, which cad-kernel 0.21.0 refuses in that order ("a shell of the
//! arrangement is kept on one side of a cylinder and dropped on another: two of its cells joined
//! round an edge").
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, kept, lofted, prism};
use cad_kernel::nurbs::Net;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 8.;
const HALF: f64 = 10.;
/// Below the saddle's lowest corner, −2.5: the block under the patch reaches down past the floor.
const BELOW: f64 = -5.;
/// The saddle point's height, and the cutting plane's.
const LEVEL: f64 = 10.;

/// The saddle's height over (x, y).
fn saddle(x: f64, y: f64) -> f64 { 10. + x * y / 8. }

/// A face of the block, linear across the section (`u`, its two rows of poles `a` and `b`, each a
/// line over x) and linear along x (`v`).
fn net(a: [[f64; 3]; 2], b: [[f64; 3]; 2]) -> Net {
    let linear = vec![0., 0., 1., 1.];
    Net { du: 1, dv: 1, uknots: linear.clone(), vknots: linear, poles: vec![a.to_vec(), b.to_vec()],
          weights: None }
}

pub fn build() -> Result<Brep, String> {
    let floor = |y: f64| [[-HALF, y, BELOW], [HALF, y, BELOW]];
    let top = |y: f64| [[-HALF, y, saddle(-HALF, y)], [HALF, y, saddle(HALF, y)]];
    // the block under the whole patch, lofted along x: its section in the plane x = const walked
    // counter-clockwise about +x, along the floor to y = 10, up that side, back under the saddle
    // (the bilinear patch, its rows the lines at y = ±10) and down the side at y = −10
    let nets = vec![
        net(floor(-HALF), floor(HALF)),
        net(floor(HALF), top(HALF)),
        net(top(HALF), top(-HALF)),
        net(top(-HALF), floor(-HALF)),
    ];
    let under = lofted([0., 0., 0.], [1., 0., 0.], [-HALF, HALF], vec![nets])?;
    // what lies on and below the plane z = 10, which meets the saddle in the x and y axes raised
    // to it, crossing at the saddle point
    let under = kept(&under, [0., 0., LEVEL], [0., 0., -1.], 0.)?;
    // the drum's cylinder, from the floor up past the saddle: in common, it trims the patch
    let drum = prism(&disc([0., 0., 0.], [0., 0., 1.], RADIUS), 0., 30.)?;
    Ok(combined(&under, &drum, Op::Common, 0.)?)
}
