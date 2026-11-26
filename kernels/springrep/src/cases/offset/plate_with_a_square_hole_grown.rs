//! The plate from (0, 0, 0) to (10, 10, 2) with the square hole from (3, 3) to (7, 7) through it,
//! grown by 1: springrep's own offset (convex edges rounded, the hole's four concave edges run on
//! until their faces meet).
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;
use crate::shared::offset::grown;

pub fn build() -> Result<Brep, String> {
    let plate = between([0., 0., 0.], [10., 10., 2.], "plate")?;
    // the hole's cutter runs past both faces
    let hole = between([3., 3., -1.], [7., 7., 3.], "hole")?;
    let holed = difference(&plate, &hole).map_err(e)?;
    grown(&holed, 1.)
}
