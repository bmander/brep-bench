//! The plate from (−20, −15, 0) to (20, 15, 4) with a window through it, the rectangle from
//! (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3: the window narrows and its
//! rounded corners close up square.
use crate::sketch::{block, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let plate = block([-20., -15., 0.], [20., 15., 4.])?;
    // the window's punch runs 1 past both faces of the plate
    let window = prism(&rounded_rectangle([-10., -7.], [10., 7.], 2., -1.), 0., 6.)?;
    grown(&combined(&plate, &window, Op::Cut, 0.)?, 3., Join::Round)
}
