//! A block (−10, −10, 0)–(10, 10, 10) united with a cone on z, its apex at the origin on the block's
//! bottom face and its base the disc of radius 5 at z = 15.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::cone;
use cad_kernel::topo::Brep;

const HALF_SIDE: f64 = 10.;
const HEIGHT: f64 = 10.;
const CONE_RADIUS: f64 = 5.;
const CONE_TOP: f64 = 15.;

pub fn build() -> Result<Brep, String> {
    let stock = block([-HALF_SIDE, -HALF_SIDE, 0.], [HALF_SIDE, HALF_SIDE, HEIGHT])?;
    // the cone on z, its tip at the origin, where it touches the block's bottom face at a single
    // point, widening to its base at the top
    let cone = cone([0., 0., 0.], [0., 0., 1.], 0., CONE_RADIUS, CONE_TOP)?;
    Ok(combined(&stock, &cone, Op::Union, 0.)?)
}
