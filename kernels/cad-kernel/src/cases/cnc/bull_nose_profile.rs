//! A plate, the box (0, 0, −16)–(120, 120, 0), with a groove cut round a six-lobed flower by a
//! bull-nose mill — radius 4, its corner rounded to 1.5 — in three laps, its tip 4, 8 and 10 below
//! the top: lobes of radius 12 about points 30 from (60, 60), valleys tangent to them about points
//! 40 from it, between.
use crate::cnc::{cutter, Cutter, Program};
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;

const CENTRE: [f64; 2] = [60., 60.];
const LOBE: f64 = 12.;
const MILL: Cutter = Cutter::Bull { radius: 4., corner: 1.5, length: 25. };
const START: [f64; 3] = [102., 60., 5.];
const LAPS: [f64; 3] = [-4., -8., -10.];
/// How closely the mill is meshed for the sweep's membership reads: a hundredth of its corner.
const BAR: f64 = 0.015;

/// The point `r` from the flower's centre at `degrees` from +x.
fn about(r: f64, degrees: f64) -> [f64; 2] {
    let a = degrees.to_radians();
    [CENTRE[0] + r * a.cos(), CENTRE[1] + r * a.sin()]
}

/// One lap at height `z` from the lobe at 0°'s outermost point: each lobe anticlockwise to its
/// tangent point with the next valley, each valley clockwise to its tangent point with the next
/// lobe, the tangent points on the lines between their centres.
fn lap(p: &mut Program, z: f64) {
    let valley = (2500. - 1200. * 3f64.sqrt()).sqrt() - LOBE;
    let toward = |from: [f64; 2], to: [f64; 2], r: f64| {
        let d = [to[0] - from[0], to[1] - from[1]];
        let l = d[0].hypot(d[1]);
        [from[0] + r * d[0] / l, from[1] + r * d[1] / l, z]
    };
    for k in 0..6 {
        let lobe = about(30., 60. * k as f64);
        let dip = about(40., 60. * k as f64 + 30.);
        let next = about(30., 60. * (k + 1) as f64);
        p.arc(lobe, toward(lobe, dip, LOBE), true, 0);
        p.arc(dip, toward(dip, next, valley), false, 0);
    }
    let first = about(30., 0.);
    p.arc(first, [first[0] + LOBE, first[1], z], true, 0);
}

pub fn build() -> Result<Brep, String> {
    let stock = block([0., 0., -16.], [120., 120., 0.])?;
    let mill = cutter(MILL, START)?;
    let mut program = Program::from(START);
    for z in LAPS {
        program.line([START[0], START[1], z]);
        lap(&mut program, z);
    }
    program.line(START);
    let size = stock.size();
    let (path, roll) = program.path(tolerance(size), size)?;
    cut_from(&stock, &mill, &path, roll, BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
