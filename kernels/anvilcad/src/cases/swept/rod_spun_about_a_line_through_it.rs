//! What a rod (radius 0.5 about the x axis from x = −3 to 5) fills as it turns 120° about the z
//! axis, which crosses it.
use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{line, rod, swung, turn_z, union, v2};

/// The half section an arm of the rod reaching `r` from the axis covers: from the axis out to the
/// sphere of radius √(r² + 0.5²) its far rim reaches, within |z| ≤ 0.5.
fn arm(r: f64) -> Vec<Segment> {
    let far = (r * r + 0.25).sqrt();
    let a = (0.5 / far).asin();
    vec![
        line(v2(0., -0.5), v2(far * a.cos(), -0.5)),
        Segment::Arc { center: v2(0., 0.), radius: far, start: -a, sweep: 2. * a },
        line(v2(far * a.cos(), 0.5), v2(0., 0.5)),
        line(v2(0., 0.5), v2(0., -0.5)),
    ]
}

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Each arm turned about the axis fills the revolve of what
    // it covers in a half-plane: the long arm's fan from 0° to 120°, the short one's from 180° to
    // 300°; and the rod itself where it starts and stops
    let r = rod([-3., 0., 0.], [5., 0., 0.], 0.5)?;
    let with_fans = union(&union(&r, &swung(arm(5.), 0., 120.)?)?, &swung(arm(3.), 180., 120.)?)?;
    union(&with_fans, &r.transformed(&turn_z(120.)))
}
