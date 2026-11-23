//! A dog-bone plate with a round boss standing on one of its lobes, the boss exactly as wide as
//! the lobe and on the same axis. The plate's outline, in the plane z = 0, is the lobes of radius
//! 10 about (−30, 0) and (30, 0), joined by a waist whose sides are y = 4 and y = −4, each side
//! running into each lobe through a concave blend of radius 6 tangent to both (its centre at
//! y = ±10, 16 from the lobe's); the plate is that outline extruded from z = 0 to 10. The boss is
//! the cylinder of radius 10 about the vertical through (30, 0) from z = 0 to 30, united with the
//! plate: where the lobe's outer arc runs, the boss's side and the plate's are one cylinder.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, TAU};

const UP: V = [0., 0., 1.];
/// The lobes' centres' distance from the origin along x, and their radius.
const LOBE_AT: f64 = 30.;
const LOBE: f64 = 10.;
/// The waist's half width, and the blends' radius.
const WAIST: f64 = 4.;
const BLEND: f64 = 6.;
const PLATE: f64 = 10.;
const BOSS_HEIGHT: f64 = 30.;

/// The arc of radius `r` about (`c`, 0) in the plane z = 0, counter-clockwise from `a0` to `a1`.
fn arc(c: [f64; 2], r: f64, a0: f64, a1: f64) -> ProfileEdge {
    let frame = Frame::new([c[0], c[1], 0.], UP, [1., 0., 0.]);
    ProfileEdge::Arc { frame, r, span: Some([a0, a1]) }
}

pub fn build() -> Result<Brep, String> {
    // each blend's centre: `WAIST + BLEND` off the axis, `LOBE + BLEND` from its lobe's centre
    let (by, reach) = (WAIST + BLEND, LOBE + BLEND);
    let bx = LOBE_AT - (reach * reach - by * by).sqrt();
    // where a blend touches a lobe, seen from the lobe's centre (`toward`) and the blend's (`back`)
    let toward = by.atan2(bx - LOBE_AT);
    let back = (-by).atan2(LOBE_AT - bx);
    let (r, l) = ([LOBE_AT, 0.], [-LOBE_AT, 0.]);
    // the outline round from the right lobe, counter-clockwise: lobe, blend, side, blend, …
    let outline = vec![
        arc(r, LOBE, -toward, toward),
        arc([bx, by], BLEND, -FRAC_PI_2, back),
        ProfileEdge::Line { a: [bx, WAIST, 0.], b: [-bx, WAIST, 0.] },
        arc([-bx, by], BLEND, -(TAU / 2. + back), -FRAC_PI_2),
        arc(l, LOBE, TAU / 2. - toward, TAU / 2. + toward),
        arc([-bx, -by], BLEND, FRAC_PI_2, TAU / 2. + back),
        ProfileEdge::Line { a: [-bx, -WAIST, 0.], b: [bx, -WAIST, 0.] },
        arc([bx, -by], BLEND, -back, FRAC_PI_2),
    ];
    let profile = Profile { origin: [0.; 3], normal: UP, loops: vec![outline], names: Vec::new() };
    let plate = prism(&profile, 0., PLATE)?;
    // the boss: the right lobe's own circle, swept up past the plate's top
    let boss = prism(&disc([LOBE_AT, 0., 0.], UP, LOBE), 0., BOSS_HEIGHT)?;
    Ok(combined(&plate, &boss, Op::Union, 0.)?)
}
