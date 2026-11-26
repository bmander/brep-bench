//! The block from (0, 0, 0) to (20, 20, 6) with the square pocket from (6, 6) to (14, 14) cut down
//! from its top to a floor at z = 3, grown by 1: springrep's own offset (convex edges rounded, the
//! pocket's concave edges run on until their faces meet).
use springrep::boolean::difference;
use springrep::topo::Brep;

use crate::shared::boolean::between;
use crate::shared::e;
use crate::shared::offset::grown;

pub fn build() -> Result<Brep, String> {
    let block = between([0., 0., 0.], [20., 20., 6.], "block")?;
    // the pocket's cutter runs out past the top
    let pocket = between([6., 6., 3.], [14., 14., 7.], "pocket")?;
    let pocketed = difference(&block, &pocket).map_err(e)?;
    grown(&pocketed, 1.)
}
