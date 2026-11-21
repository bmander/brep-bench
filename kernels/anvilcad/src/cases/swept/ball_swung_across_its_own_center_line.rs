//! What a ball of radius 2 about (1, 0, 0) fills as it turns 120° about the z axis, which passes
//! through it.
use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{ball, line, swung, turn_z, union, v2};

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. The ball turned about an axis through it fills the
    // revolve of what it covers in a half-plane through the axis: at each height every distance
    // from the axis out to the ball's farthest, which is its disk on the half-plane's side of the
    // axis (its arc from the axis round to the axis, and the axis between); and itself where it
    // starts and stops
    let a = (1f64 / 2.).acos();
    let fan = swung(
        vec![
            Segment::Arc { center: v2(1., 0.), radius: 2., start: -(std::f64::consts::PI - a), sweep: 2. * (std::f64::consts::PI - a) },
            line(v2(0., 3f64.sqrt()), v2(0., -3f64.sqrt())),
        ],
        0.,
        120.,
    )?;
    let b = ball([1., 0., 0.], 2.)?;
    union(&union(&fan, &b)?, &b.transformed(&turn_z(120.)))
}
