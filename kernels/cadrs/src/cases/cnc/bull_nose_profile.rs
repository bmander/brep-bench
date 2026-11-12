//! A groove profiled round a six-lobed flower by a bull-nose mill in three step-downs. The plate
//! is the box from (0, 0, −16) to (120, 120, 0); a bull-nose mill of radius 4, its end corner
//! rounded to 1.5, upright, plunges at (102, 60) and runs anticlockwise round the flower (six
//! lobes of radius 12 about points 30 from (60, 60), six valleys tangent to them about points
//! 40 from it) at z = −4, −8 and −10, then rises.
//!
//! Each lap's groove is the mill's section through its axis (8 wide, its two bottom corners
//! rounded) carried upright round the flower, cadrs's Sweep along a path: its walls cylinders,
//! its corners tori. The plunges and the rise at (102, 60), a point of the flower, are within
//! the laps' grooves. The plate less the three, each cut in turn as the program cuts them.
use crate::bench::{arc, block, cut, p2, p3};
use crate::shared::cnc::{across, round_the_path};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{BodyId, Curve2, DefaultKernel};

/// The flower from (102, 60) anticlockwise and back: thirteen arcs, the lobe at 0° in two.
fn flower() -> Vec<Curve2> {
    let d = (2500.0 - 1200.0 * 3f64.sqrt()).sqrt();
    let about = |r: f64, degrees: f64| {
        let a = degrees.to_radians();
        (60.0 + r * a.cos(), 60.0 + r * a.sin())
    };
    let lobe = |k: usize| about(30.0, 60.0 * k as f64);
    let valley = |k: usize| about(40.0, 30.0 + 60.0 * k as f64);
    // The way from one centre to another, in degrees.
    let toward = |a: (f64, f64), b: (f64, f64)| (b.1 - a.1).atan2(b.0 - a.0).to_degrees();
    let mut path = Vec::new();
    for k in 0..6 {
        let (l, v, next) = (lobe(k), valley(k), lobe((k + 1) % 6));
        // Along the lobe to where it touches its valley (from its outermost point, or from
        // where the valley before left it), anticlockwise.
        let from = if k == 0 { 0.0 } else { toward(l, valley(k - 1)) };
        let to = toward(l, v);
        path.push(arc(p2(l.0, l.1), 12.0, from, (to - from).rem_euclid(360.0)));
        // Round the valley to the next lobe, clockwise.
        let (from, to) = (toward(v, l), toward(v, next));
        path.push(arc(p2(v.0, v.1), d - 12.0, from, -(from - to).rem_euclid(360.0)));
    }
    // Back along the lobe at 0° to its outermost point.
    let from = toward(lobe(0), valley(5));
    path.push(arc(p2(90.0, 60.0), 12.0, from, (0.0 - from).rem_euclid(360.0)));
    path
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let mut plate = block(k, p3(0.0, 0.0, -16.0), p3(120.0, 120.0, 0.0))?;
    let w = 2f64.sqrt() / 2.0;
    for depth in [-4.0, -8.0, -10.0] {
        // The mill's section where the lap starts, at (102, 60) heading along +y.
        let section = Section::on(across(102.0, 60.0, (0.0, 1.0)))
            .line((-2.5, depth), (2.5, depth))
            .conic((2.5, depth), (4.0, depth), w, (4.0, depth + 1.5))
            .line((4.0, depth + 1.5), (4.0, 5.0))
            .line((4.0, 5.0), (-4.0, 5.0))
            .line((-4.0, 5.0), (-4.0, depth + 1.5))
            .conic((-4.0, depth + 1.5), (-4.0, depth), w, (-2.5, depth));
        let groove = round_the_path(k, &section.sketch(), flower())?;
        plate = cut(k, plate, groove)?;
    }
    Ok(plate)
}
