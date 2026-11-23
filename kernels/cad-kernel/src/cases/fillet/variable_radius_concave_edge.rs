//! An L-shaped block 10 deep — the L through (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12), as
//! (x, z), extruded from y = 0 to y = 10 — its one inside edge, from (6, 0, 4) to (6, 10, 4),
//! rounded with a radius that grows linearly along it, r = 1 + y/5, filling the inside corner.
//!
//! Built round the kernel's lack of a variable-radius round (`fillet::roll` rolls a ball of one
//! radius; nothing in `fillet`, `rounds` or `corners` takes a radius that changes along an edge),
//! from what the case says the round is: its sections square to the edge are quarter circles of
//! radius r(y) about (6 + r, y, 4 + r), so it is part of an oblique circular cone — the right
//! frustum `build::cone` makes, of radius 1 at y = 0 and 3 at y = 10, sheared across its axis by
//! `affine::Affine` (`Brep::mapped`) until its centres run along that line. The corner the round
//! fills lies short of the two planes x = 6 + r and z = 4 + r through the cone's axis and the lines
//! it touches the faces along: that corner is joined to the block and the quarter of the cone
//! between those planes cut away again.
use crate::sketch::{block, polygon};
use cad_kernel::affine::Affine;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, kept, prism};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const DEPTH: f64 = 10.;
/// The inside edge, at x = 6, z = 4, along y.
const EDGE: [f64; 2] = [6., 4.];
/// The round's radius at y = 0 and at y = DEPTH.
const R: [f64; 2] = [1., 3.];

pub fn build() -> Result<Brep, String> {
    // the L in the xz plane, swept along +y (walked this way round its profile faces +y)
    let l = polygon(&[[0., 0., 0.], [0., 0., 12.], [6., 0., 12.], [6., 0., 4.], [20., 0., 4.],
                      [20., 0., 0.]]);
    let body = prism(&l, 0., DEPTH)?;
    // the radius r(y) = R0 + slope y, and the plane x = 6 + r(y), i.e. x − slope y = 6 + R0, and
    // its twin in z: each kept on the edge's side, (−1, slope, 0) or (0, slope, −1)
    let slope = (R[1] - R[0]) / DEPTH;
    let [x, z] = EDGE;
    let axis = [x + R[0], 0., z + R[0]];
    let corner = |b: &Brep| -> Result<Brep, String> {
        let b = kept(b, axis, unit([-1., slope, 0.]), 0.)?;
        kept(&b, axis, unit([0., slope, -1.]), 0.)
    };
    // the corner: the block over it out to the greatest radius, short of the two planes
    let reach = [x + R[1], DEPTH, z + R[1]];
    let fill = corner(&block([x, 0., z], reach)?)?;
    // the cone: radius R0 about the y axis at y = 0 and R1 at y = DEPTH, sheared so its section
    // at y is centred at (6 + r(y), y, 4 + r(y))
    let shear = Affine::new([[1., slope, 0.], [0., 1., 0.], [0., slope, 1.]], axis)?;
    let round = cone([0.; 3], [0., 1., 0.], R[0], R[1], DEPTH)?.mapped(&shear);
    let filled = combined(&body, &fill, Op::Union, 0.)?;
    Ok(combined(&filled, &corner(&round)?, Op::Cut, 0.)?)
}
