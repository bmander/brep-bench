//! The plate from (0, 0, −16) to (120, 120, 0), with a groove milled round a six-lobed flower by a
//! bull-nose mill of radius 4, its end corner rounded to 1.5, its axis upright: three laps round
//! the flower, the tip at z = −4, −8 and −10. The flower is smooth, of arcs about the upright line
//! through (60, 60): six lobes of radius 12 about points 30 out at 0°, 60°, … run anticlockwise,
//! and between them six valleys about points 40 out at 30°, 90°, …, each tangent to the lobes
//! beside it, run clockwise. Each lap starts at (102, 60), the outermost point of the lobe at 0°.
//! The plate less everything the mill passes through.
//!
//! The mill is its half section revolved about its axis. Each arc of each lap is kentron's sweep
//! of a solid of the mill (`shared::cnc`: a turn about the arc's centre), all cut from the plate at
//! once. The plunges to each lap's start and the rise after the last run along the mill's own axis
//! at (102, 60), a point of the path, from above each lap's depth: they pass through nothing the
//! mill at that depth, 25 tall, does not, so they are not swept.
use crate::bench::p;
use crate::shared::cnc::{along, cut, swept, upright};
use kentron::primitives::Cuboid;
use kentron::sketch::Step::{self, Arc, Heading, Start, To};
use kentron::topology::{Model, Shape};

/// The mill's half section from its tip: its flat end out to 2.5, rounded up by a quarter circle
/// of radius 1.5 to its side, 4 out, and up to 25 above the tip.
const HALF: [Step; 5] = [Start(0.0, 0.0), To(2.5, 0.0), Arc { radius: 1.5, angle: 90.0 }, To(4.0, 25.0), To(0.0, 25.0)];

/// The flower from (102, 60), heading +y: each lobe turns left through the angle between its
/// valleys' directions from its centre, each valley right through what closes the turn.
fn flower() -> Vec<Step> {
    let (lobe, valley_center) = (12.0, 40.0);
    // From the lobe at 0°, centre (90, 60), toward the valley at 30°: where they touch.
    let (dx, dy) = (valley_center * 30f64.to_radians().cos() - 30.0, valley_center * 30f64.to_radians().sin());
    let valley = dx.hypot(dy) - lobe;
    let half_lobe = dy.atan2(dx).to_degrees();
    // six lobes turn 2·half_lobe left and six valleys turn right: once round in all
    let valley_turn = 2.0 * half_lobe - 60.0;
    let mut steps = vec![Start(102.0, 60.0), Heading(90.0), Arc { radius: lobe, angle: half_lobe }];
    for k in 0..6 {
        steps.push(Arc { radius: -valley, angle: valley_turn });
        let turn = if k < 5 { 2.0 * half_lobe } else { half_lobe };
        steps.push(Arc { radius: lobe, angle: turn });
    }
    steps
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, -16.0), p(120.0, 120.0, 0.0)).build(m)?;
    let flower = flower();
    let moves: Vec<_> = [-4.0, -8.0, -10.0].into_iter().flat_map(|z| along(&flower, z)).collect();
    let cuts = swept(m, |m| upright(m, &HALF), &moves)?;
    cut(m, plate, &cuts)
}
