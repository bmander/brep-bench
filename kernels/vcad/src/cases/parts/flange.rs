//! A round mounting flange on the z axis: a plate of radius 32 from z = 0 to z = 6, a hub of
//! radius 15 on it up to z = 16, a bore of radius 8 right through, and six bolt holes of radius 3
//! through the plate on the circle of radius 24, one every 60° from (24, 0).
use crate::solids::SEGMENTS;
use vcad_kernel::Solid;

const PLATE_RADIUS: f64 = 32.0;
const PLATE_TOP: f64 = 6.0;
const HUB_RADIUS: f64 = 15.0;
const HUB_TOP: f64 = 16.0;
const BORE: f64 = 8.0;
const HOLES: usize = 6;
const HOLE_RADIUS: f64 = 3.0;
const HOLE_CIRCLE: f64 = 24.0;

pub fn build() -> Result<Solid, crate::Refusal> {
    let plate = Solid::cylinder(PLATE_RADIUS, PLATE_TOP, SEGMENTS);
    let hub = Solid::cylinder(HUB_RADIUS, HUB_TOP - PLATE_TOP, SEGMENTS)
        .translate(0.0, 0.0, PLATE_TOP);
    // the bore and each hole its own cut, each cylinder running 1 past both faces it opens
    let bore = Solid::cylinder(BORE, HUB_TOP + 2.0, SEGMENTS).translate(0.0, 0.0, -1.0);
    let mut flange = plate.try_union(&hub)?.try_difference(&bore)?;
    for i in 0..HOLES {
        let turn = (60.0 * i as f64).to_radians();
        let hole = Solid::cylinder(HOLE_RADIUS, PLATE_TOP + 2.0, SEGMENTS).translate(
            HOLE_CIRCLE * turn.cos(),
            HOLE_CIRCLE * turn.sin(),
            -1.0,
        );
        flange = flange.try_difference(&hole)?;
    }
    Ok(flange)
}
