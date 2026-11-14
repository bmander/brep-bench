//! A groove profiled round a cross by a flat end mill in three step-downs. The plate is the box
//! from (0, 0, −15) to (120, 90, 0); a flat end mill of radius 3, 20 long from its tip, upright,
//! plunges at (60, 10) and runs anticlockwise round the outline of a cross (a bar 80 by 30 across a
//! bar 30 by 70, every corner rounded to 6) at z = −3, −6 and −9, then rises. The solid is the plate
//! less everything the mill passes through, each move swept by the kernel's sweep of a solid.
use crate::solids::cnc::{cut, rounded_loop, Cutter, Program};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

const CROSS: [[f64; 2]; 12] = [
    [45.0, 10.0], [75.0, 10.0], [75.0, 30.0], [100.0, 30.0], [100.0, 60.0], [75.0, 60.0],
    [75.0, 80.0], [45.0, 80.0], [45.0, 60.0], [20.0, 60.0], [20.0, 30.0], [45.0, 30.0],
];

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, 0.0, -15.0), 120.0, 90.0, 15.0)?;
    let start = p(60.0, 10.0, 5.0);
    let mut mill = Program::new(Cutter::flat(3.0, 20.0), start, 0.0);
    for z in [-3.0, -6.0, -9.0] {
        mill.line(p(60.0, 10.0, z))?;
        rounded_loop(&mut mill, &CROSS, 6.0, z)?;
    }
    mill.line(start)?;
    cut(plate, vec![mill])
}
