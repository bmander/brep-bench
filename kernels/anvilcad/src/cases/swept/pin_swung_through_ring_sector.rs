//! The ring sector 8 ≤ ρ ≤ 12, |z| ≤ 1, 0° ≤ φ ≤ 60° less what a pin (radius 0.5 about the x
//! axis from x = 6 to 11) passes through, turned about the z axis from −140° to 140°.
use anvil_brep::Solid;
use anvil_brep::boolean::{BoolOp, boolean};
use anvil_brep::profile::Segment;

use crate::shared::{e, line, rect, rod, swung, turn_z, union, v2};

pub fn build() -> Result<Vec<Solid>, String> {
    let sector = swung(rect((8., -1.), (12., 1.)).segments, 0., 60.)?;
    // anvil-brep has no sweep of a solid. The pin turned about an axis it meets fills the revolve
    // of what it covers in a half-plane through the axis, from 6 out to the sphere of radius
    // √(11² + 0.5²) its far rim reaches, within |z| ≤ 0.5, and itself where it starts and stops
    let r = (121.25f64).sqrt();
    let a = (0.5 / r).asin();
    let fan = swung(
        vec![
            line(v2(6., -0.5), v2(r * a.cos(), -0.5)),
            Segment::Arc { center: v2(0., 0.), radius: r, start: -a, sweep: 2. * a },
            line(v2(r * a.cos(), 0.5), v2(6., 0.5)),
            line(v2(6., 0.5), v2(6., -0.5)),
        ],
        -140.,
        280.,
    )?;
    let pin = rod([6., 0., 0.], [11., 0., 0.], 0.5)?;
    let slot = union(&union(&fan, &pin.transformed(&turn_z(-140.)))?, &pin.transformed(&turn_z(140.)))?;
    boolean(&sector, &slot, BoolOp::Subtract).map_err(e)
}
