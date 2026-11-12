//! A round blank, the cylinder of radius 50 about z from z = 0 to 45, machined into a dome (the
//! ball of radius 40 about the origin) by a five-axis program of four cutters: roughed in level
//! rings by a bull-nose mill, finished by a ball-end mill held square to the dome along 180
//! meridians and round 15 circles near its pole, then spot-faced and drilled at twelve points
//! square to it. Every move is a slide, a turn about z or a turn about a level line through the
//! origin.
use crate::cnc::{cutter, standing, Cutter, Program};
use cad_kernel::build::{disc, prism};
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::util::space::scale;
use std::f64::consts::{FRAC_PI_4, TAU};

const ROUGHER: Cutter = Cutter::Bull { radius: 8., corner: 2., length: 40. };
const FINISHER: Cutter = Cutter::Ball { radius: 4., length: 40. };
const FACER: Cutter = Cutter::Flat { radius: 5., length: 40. };
const DRILL: Cutter = Cutter::Drill { radius: 2.5, half: FRAC_PI_4, length: 40. };
const DOME: f64 = 40.;
/// How far from the origin a leaning cutter is carried between cuts, and the seats' and holes'.
const CLEAR: f64 = 90.;
const NEAR: f64 = 70.;
const UP: [f64; 3] = [0., 0., 1.];
/// How closely each cutter is meshed for the sweep's membership reads.
const BAR: f64 = 0.02;

/// The dome's outward normal at latitude `lat` and longitude `lon`, in degrees.
fn normal(lat: f64, lon: f64) -> [f64; 3] {
    let (l, p) = (lat.to_radians(), lon.to_radians());
    [l.cos() * p.cos(), l.cos() * p.sin(), l.sin()]
}

/// The roughing: at each level the circles ρ + 9 + 12m short of 58, the largest first.
fn roughing() -> Program {
    let rings: Vec<(f64, f64)> = [41., 36., 31., 26., 21., 16., 11., 6.].iter().flat_map(|&z: &f64| {
        let rho = (DOME * DOME - z * z).max(0.).sqrt();
        let radii: Vec<f64> = (0..).map(|m| rho + 9. + 12. * m as f64).take_while(|&r| r < 58.).collect();
        radii.into_iter().rev().map(move |r| (r, z))
    }).collect();
    let mut p = Program::from([rings[0].0, 0., 50.]);
    for (r, z) in rings {
        p.line([r, 0., 50.]);
        p.line([r, 0., z]);
        p.arc([0., 0.], [r, 0., z], true, 0);
        p.line([r, 0., 50.]);
    }
    p
}

/// The finishing: 180 meridian passes from latitude 3° to 75°, 15 round the pole, and its touch.
fn finishing() -> Program {
    let mut p = Program::placed(standing(scale(normal(3., 0.), CLEAR), normal(3., 0.)));
    let (climb, reach) = (72f64.to_radians(), CLEAR - DOME);
    for k in 0..180 {
        let lon = (2 * k) as f64;
        let line = [lon.to_radians().sin(), -lon.to_radians().cos(), 0.];
        p.along_axis(-reach).turn([0.; 3], line, climb).along_axis(reach)
         .turn([0.; 3], line, -climb).turn([0.; 3], UP, 2f64.to_radians());
    }
    p.turn([0.; 3], [0., -1., 0.], climb);
    for _ in 75..90 {
        p.along_axis(-reach).turn([0.; 3], UP, TAU).along_axis(reach)
         .turn([0.; 3], [0., -1., 0.], 1f64.to_radians());
    }
    p.along_axis(-reach).along_axis(reach);
    p
}

/// The seats or the holes: at each of the twelve points at latitude 40°, in along the normal to
/// `depth` from the origin and out again, then round the z axis by 30° to the next.
fn square_to_the_dome(depth: f64) -> Program {
    let mut p = Program::placed(standing(scale(normal(40., 15.), NEAR), normal(40., 15.)));
    for k in 0..12 {
        p.along_axis(depth - NEAR).along_axis(NEAR - depth);
        if k < 11 { p.turn([0.; 3], UP, 30f64.to_radians()); }
    }
    p
}

pub fn build() -> Result<Brep, String> {
    let mut part = prism(&disc([0.; 3], UP, 50.), 0., 45.)?;
    let size = part.size();
    let steps = [(ROUGHER, roughing()), (FINISHER, finishing()),
                 (FACER, square_to_the_dome(38.5)), (DRILL, square_to_the_dome(30.))];
    for (tool, program) in steps {
        let cutter = cutter(tool, program.drawn())?;
        let (path, roll) = program.path(tolerance(size), size)?;
        part = cut_from(&part, &cutter, &path, roll, BAR, tolerance(size))
            .map_err(|why| format!("a cutter's sweep not built: {why:?}"))?;
    }
    Ok(part)
}
