//! Two balls of radius 5, about the origin and about (9.999999, 0, 0): the lens common to both,
//! a millionth thick.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const APART: f64 = 9.999999;

/// The ball of radius `RADIUS` about `centre`, its poles on the vertical through it.
fn ball(centre: V) -> Result<Brep, String> {
    sphere(centre, [0., 0., 1.], RADIUS)
}

pub fn build() -> Result<Brep, String> {
    let (first, second) = (ball([0., 0., 0.])?, ball([APART, 0., 0.])?);
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
