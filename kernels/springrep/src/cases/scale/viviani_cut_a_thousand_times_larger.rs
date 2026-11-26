//! `tangent/viviani-cut` a thousand times larger: the ball of radius 10000 about the origin and
//! the cylinder of radius 5000 about the line x = 5000, y = 0 from z = -12000 to 12000, which
//! touches the sphere from inside at (10000, 0, 0) and meets it in Viviani's curve: the ball less the cylinder.
//! Built as the original is, every length times 1000.
use springrep::boolean::difference;
use springrep::primitive::sphere;
use springrep::topo::Brep;

use crate::shared::tangent::rod;
use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the ball's poles on the y axis and its seam through (-10000, 0, 0), off the curve
    let ball = sphere(&frame([0., 0., 0.], [0., 1., 0.], [-1., 0., 0.])?, 10000., "ball").map_err(e)?;
    // the cylinder's seam on the line x = 0, y = 0, away from (10000, 0, 0)
    let bore = rod([5000., 0., -12000.], [0., 0., 1.], [-1., 0., 0.], 5000., 24000., "cylinder")?;
    difference(&ball, &bore).map_err(e)
}
