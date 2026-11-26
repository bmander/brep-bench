//! A barrel, the ball of radius 5 about the origin kept between the planes z = −3 and z = 3, shrunk
//! by 1: the ball of radius 4 kept between z = −2 and z = 2, its edges sharp.
use cad_kernel::build::{kept, sphere};
use cad_kernel::offset::{shrunk, Join};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 5.;
const HALF_HEIGHT: f64 = 3.;

pub fn build() -> Result<Brep, String> {
    let ball = sphere([0., 0., 0.], [0., 0., 1.], RADIUS)?;
    // the ball kept under the plane z = 3, then over the plane z = −3
    let barrel = kept(&ball, [0., 0., HALF_HEIGHT], [0., 0., -1.], 0.)?;
    let barrel = kept(&barrel, [0., 0., -HALF_HEIGHT], [0., 0., 1.], 0.)?;
    shrunk(&barrel, 1., Join::Round)
}
