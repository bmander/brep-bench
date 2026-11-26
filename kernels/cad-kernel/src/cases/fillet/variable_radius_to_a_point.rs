//! A cube of side 10 between the origin and (10, 10, 10), its edge where the faces x = 10 and
//! y = 10 meet rounded with a radius that shrinks linearly along it to nothing: r = 2 − z/5, 2 at
//! the bottom and 0 at the top, where the round comes to a point at the cube's corner.
//!
//! Built round the kernel's lack of a variable-radius round shrinking to nothing
//! (`fillet::roll_varying` rolls a ball whose radius changes along the edge, but refuses one of no
//! size, which it has at the top, where its faces' offsets meet the edge itself), from what the case says the round is: its sections square to the edge are quarter circles of
//! radius r(z) about (10 − r, 10 − r, z), so it is part of an oblique circular cone — the right
//! cone `build::cone` makes, of radius 2 at its base and its apex 10 above it, sheared along its
//! axis by `affine::Affine` (`Brep::mapped`) until its centres run along that line. The corner the
//! round takes is the cube's part past the two planes x = 10 − r and y = 10 − r through the
//! cone's axis and the lines it touches the faces along; the cube is cut there and the quarter of
//! the cone between those planes put back.
use crate::sketch::block;
use cad_kernel::affine::Affine;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{cone, kept};
use cad_kernel::topo::Brep;
use cad_kernel::util::space::unit;

const SIDE: f64 = 10.;
/// The round's radius at the bottom face, z = 0; it falls to nought at the top.
const R0: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0.; 3], [SIDE; 3])?;
    // the radius r(z) = R0 + slope z, and the plane x = SIDE − r(z), i.e. x + slope z = SIDE − R0,
    // and its twin in y: each kept on the edge's side, (1, 0, slope) or (0, 1, slope)
    let slope = -R0 / SIDE;
    let at = SIDE - R0;
    let corner = |b: &Brep| -> Result<Brep, String> {
        let b = kept(b, [at, at, 0.], unit([1., 0., slope]), 0.)?;
        kept(&b, [at, at, 0.], unit([0., 1., slope]), 0.)
    };
    // the cone: radius R0 about the z axis at z = 0, its apex SIDE above, sheared so its section
    // at height z is centred at (SIDE − r(z), SIDE − r(z))
    let shear = Affine::new([[1., 0., -slope], [0., 1., -slope], [0., 0., 1.]], [at, at, 0.])?;
    let round = cone([0.; 3], [0., 0., 1.], R0, 0., SIDE)?.mapped(&shear);
    let cut = combined(&cube, &corner(&cube)?, Op::Cut, 0.)?;
    Ok(combined(&cut, &corner(&round)?, Op::Union, 0.)?)
}
