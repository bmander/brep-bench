//! A plate, the box (0, 0, −15)–(120, 90, 0), with a groove cut round a cross by a flat end mill
//! of radius 3 in three laps, its tip 3, 6 and 9 below the top, along the cross's outline with
//! every corner rounded to 6.
use crate::cnc::{cutter, rounded_loop, Cutter, Program};
use crate::sketch::block;
use cad_kernel::geom::tolerance;
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;

/// The cross the tip runs round, anticlockwise from the middle of its lowest side.
const CROSS: [[f64; 2]; 12] = [[45., 10.], [75., 10.], [75., 30.], [100., 30.], [100., 60.],
    [75., 60.], [75., 80.], [45., 80.], [45., 60.], [20., 60.], [20., 30.], [45., 30.]];
const ROUND: f64 = 6.;
const MILL: Cutter = Cutter::Flat { radius: 3., length: 20. };
const START: [f64; 3] = [60., 10., 5.];
const LAPS: [f64; 3] = [-3., -6., -9.];
/// How closely the mill is meshed for the sweep's membership reads: a hundredth of its radius.
const BAR: f64 = 0.03;

pub fn build() -> Result<Brep, String> {
    let stock = block([0., 0., -15.], [120., 90., 0.])?;
    let mill = cutter(MILL, START)?;
    let mut program = Program::from(START);
    for z in LAPS {
        program.line([START[0], START[1], z]);
        rounded_loop(&mut program, &CROSS, ROUND, z);
    }
    program.line(START);
    let size = stock.size();
    let (path, roll) = program.path(tolerance(size), size)?;
    cut_from(&stock, &mill, &path, roll, BAR, tolerance(size))
        .map_err(|why| format!("the mill's sweep not built: {why:?}"))
}
