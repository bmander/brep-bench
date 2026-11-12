//! A plate, the box (0, 0, −20)–(160, 110, 0), machined by a 2.5-axis program of three tools: a
//! pocket — the rectangle (20, 20)–(140, 90) rounded to 12, less the disc of radius 15 about
//! (60, 55) — cleared 12 deep at three depths by a flat end mill of radius 5 in raster passes 4
//! apart and finished round its wall and its island at each; a bore of radius 9, 6 deep, in the
//! island's top by a flat end mill of radius 3; and four holes pecked by a drill of radius 4 with a
//! 90° point.
use crate::cnc::{cutter, rounded_loop, Cutter, Program};
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;

const ROUGHER: Cutter = Cutter::Flat { radius: 5., length: 30. };
const BORER: Cutter = Cutter::Flat { radius: 3., length: 30. };
const DRILL: Cutter = Cutter::Drill { radius: 4., half: std::f64::consts::FRAC_PI_4, length: 30. };
/// Where every move between cuts is made.
const CLEAR: f64 = 5.;
/// The wall path, the rectangle (25, 25)–(135, 85) rounded to 7, and the island path, the circle of
/// radius 20 about (60, 55).
const WALL: [[f64; 2]; 4] = [[25., 25.], [135., 25.], [135., 85.], [25., 85.]];
const WALL_ROUND: f64 = 7.;
const ISLAND: [f64; 2] = [60., 55.];
const ISLAND_PATH: f64 = 20.;
const DEPTHS: [f64; 3] = [-4., -8., -12.];
const HOLES: [[f64; 2]; 4] = [[8., 8.], [152., 8.], [152., 102.], [8., 102.]];
/// How closely each cutter is meshed for the sweep's membership reads.
const BAR: f64 = 0.03;

/// Where the line y = `y` enters and leaves the wall path's inside, `x` from its left end.
fn across_wall(y: f64) -> [f64; 2] {
    let (lo, hi) = (WALL[0][1] + WALL_ROUND, WALL[2][1] - WALL_ROUND);
    let inset = if y < lo { lo - y } else if y > hi { y - hi } else { 0. };
    let x = WALL_ROUND - (WALL_ROUND * WALL_ROUND - inset * inset).sqrt();
    [WALL[0][0] + x, WALL[1][0] - x]
}

/// The intervals of the line y = `y` inside the wall path and outside the island path, left to right.
fn raster(y: f64) -> Vec<[f64; 2]> {
    let [a, b] = across_wall(y);
    let dy = y - ISLAND[1];
    if dy.abs() >= ISLAND_PATH { return vec![[a, b]] }
    let half = (ISLAND_PATH * ISLAND_PATH - dy * dy).sqrt();
    vec![[a, ISLAND[0] - half], [ISLAND[0] + half, b]]
}

/// A plunge to `depth` at `at`, the moves `cut` there, and a rise, from wherever the tip is at the
/// clearance height.
fn plunged(p: &mut Program, at: [f64; 2], depth: f64, cut: impl FnOnce(&mut Program)) {
    p.line([at[0], at[1], CLEAR]);
    p.line([at[0], at[1], depth]);
    cut(p);
    let end = p.tip();
    p.line([end[0], end[1], CLEAR]);
}

/// The pocket's program: at each depth its raster passes, its wall and its island.
fn pocket() -> Program {
    let start = [ISLAND[0], ISLAND[1], CLEAR];
    let mut p = Program::from(start);
    for z in DEPTHS {
        for k in 0..15 {
            let y = 26. + 4. * k as f64;
            for [a, b] in raster(y) {
                plunged(&mut p, [a, y], z, |p| { p.line([b, y, z]); });
            }
        }
        plunged(&mut p, [80., 25.], z, |p| rounded_loop(p, &WALL, WALL_ROUND, z));
        let round = [ISLAND[0] + ISLAND_PATH, ISLAND[1]];
        plunged(&mut p, round, z, |p| { p.arc(ISLAND, [round[0], round[1], z], false, 0); });
    }
    p
}

/// The bore's program: at each depth out from the island's centre, a whole circle, and back.
fn bore() -> Program {
    let mut p = Program::from([ISLAND[0], ISLAND[1], CLEAR]);
    for z in [-3., -6.] {
        p.line([ISLAND[0], ISLAND[1], z]);
        let out = [ISLAND[0] + 6., ISLAND[1], z];
        p.line(out);
        p.arc(ISLAND, out, true, 0);
        p.line([ISLAND[0], ISLAND[1], z]);
    }
    p.line([ISLAND[0], ISLAND[1], CLEAR]);
    p
}

/// The holes' program: each pecked to −5, −10 and −14, rising to 2 between pecks.
fn holes() -> Program {
    let [x, y] = HOLES[0];
    let mut p = Program::from([x, y, CLEAR]);
    for [x, y] in HOLES {
        p.line([x, y, CLEAR]);
        for (down, up) in [(-5., 2.), (-10., 2.), (-14., CLEAR)] {
            p.line([x, y, down]);
            p.line([x, y, up]);
        }
    }
    p
}

pub fn build() -> Result<Brep, String> {
    let mut part = block([0., 0., -20.], [160., 110., 0.])?;
    let size = part.size();
    for (tool, program) in [(ROUGHER, pocket()), (BORER, bore()), (DRILL, holes())] {
        let cutter = cutter(tool, program.drawn())?;
        let (path, roll) = program.path(tolerance(size), size)?;
        part = cut_from(&part, &cutter, &path, roll, BAR, tolerance(size))
            .map_err(|why| format!("a cutter's sweep not built: {why:?}"))?;
    }
    Ok(part)
}
