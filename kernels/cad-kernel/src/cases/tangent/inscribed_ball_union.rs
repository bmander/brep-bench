//! A cube of side 10 at the origin united with its inscribed ball, radius 5 about (5, 5, 5), which
//! touches each of the cube's faces at the middle of the face.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

const SIDE: f64 = 10.;
const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // the ball: its half disc in the vertical plane through its centre, parallel to xz, turned
    // once about the vertical through the centre
    let centre = [SIDE / 2.; 3];
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, [0., 0., 1.], RADIUS)?;
    Ok(combined(&cube, &ball, Op::Union, 0.)?)
}
