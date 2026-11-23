//! The plate from (0, 0, −15) to (120, 90, 0), with a groove milled round a cross by a flat end
//! mill of radius 3 (a solid cylinder 20 up from its tip), its axis upright: three laps round the
//! cross's outline, its twelve corners rounded to radius 6, anticlockwise from (60, 10), the tip
//! at z = −3, −6 and −9. The plate less everything the mill passes through.
//!
//! Each lap is the band within the mill's radius, 3, of the path, extruded 20 up from the tip
//! (`shared::cnc::flat_milled_round`). The plunges to each lap's start and the rise after the last run
//! along the mill's own axis at (60, 10), a point of the path, from above each lap's depth: they
//! pass through nothing the lap at that depth does not.
use crate::bench::p;
use crate::shared::cnc::{flat_milled_round, level_path};
use crate::shared::swept::cut_each;
use kentron::primitives::Cuboid;
use kentron::sketch::Step::{self, Arc, Forward, Start};
use kentron::topology::{Model, Shape};

/// The rounded cross from (60, 10) along +x, anticlockwise: each side less the 6 its rounds
/// take, a left turn round an outside corner and a right turn round an inside one.
const CROSS: [Step; 26] = [
    Start(60.0, 10.0),
    Forward(9.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(8.0), Arc { radius: -6.0, angle: 90.0 },
    Forward(13.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(18.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(13.0), Arc { radius: -6.0, angle: 90.0 },
    Forward(8.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(18.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(8.0), Arc { radius: -6.0, angle: 90.0 },
    Forward(13.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(18.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(13.0), Arc { radius: -6.0, angle: 90.0 },
    Forward(8.0), Arc { radius: 6.0, angle: 90.0 },
    Forward(9.0),
];

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -15.0), p(120.0, 90.0, 0.0)).build(m)?;
    let mut laps = Vec::new();
    for z in [-3.0, -6.0, -9.0] {
        let path = level_path(m, z, &CROSS)?;
        laps.push(flat_milled_round(m, path, 3.0, 20.0)?);
    }
    cut_each(m, plate, &laps)
}
