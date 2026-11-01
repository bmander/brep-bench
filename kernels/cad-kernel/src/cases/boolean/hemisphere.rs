//! A ball of radius 5 centred on the origin, with everything below the xy plane cut away.
use cad_kernel::build::{kept, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;

pub fn build() -> Result<Brep, String> {
    let centre = [0., 0., 0.];
    // the ball, its poles on the vertical through its centre
    let ball = sphere(centre, [0., 0., 1.], RADIUS)?;
    // the part above the xy plane
    kept(&ball, centre, [0., 0., 1.], 0.)
}
