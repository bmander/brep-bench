//! A block (−10, −10, −10)–(10, 10, 0) with a ball of radius 5 at the origin cut from its top.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let stock = block([-10., -10., -10.], [10., 10., 0.])?;
    let centre = [0., 0., 0.];
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, [0., 0., 1.], RADIUS)?;
    Ok(combined(&stock, &ball, Op::Cut, 0.)?)
}
