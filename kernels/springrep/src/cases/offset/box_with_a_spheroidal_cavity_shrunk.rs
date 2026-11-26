//! The cube from (−10, −10, −10) to (10, 10, 10) less the oblate spheroid of
//! `geometry/oblate-spheroid`, a closed cavity at its middle, shrunk by 1: springrep's own offset
//! by −1.
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::cases::geometry::r#oblate_spheroid;
use crate::shared::boolean::between;
use crate::shared::e;
use crate::shared::offset::shrunk;

pub fn build() -> Result<Brep, String> {
    let cube = between([-10., -10., -10.], [10., 10., 10.], "cube")?;
    let hollow = difference(&cube, &oblate_spheroid::build()?).map_err(e)?;
    shrunk(&hollow, 1.)
}
