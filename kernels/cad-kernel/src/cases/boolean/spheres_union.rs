//! Two balls of radius 5, one centred on the origin and one at (6, 0, 0), united.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::geom::V;
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

/// The ball of radius `RADIUS` about `centre`, its poles on the vertical through it.
fn ball(centre: V) -> Result<Brep, String> {
    sphere(centre, [0., 0., 1.], RADIUS)
}

pub fn build() -> Result<Brep, String> {
    let (first, second) = (ball([0., 0., 0.])?, ball([6., 0., 0.])?);
    Ok(combined(&first, &second, Op::Union, 0.)?)
}
