//! The block from (−20, −15, 0) to (20, 15, 10) with a pocket down to z = 4, the rectangle from
//! (−10, −7) to (10, 7) with its corners rounded to radius 2, grown by 3, more than the corners'
//! radius: the pocket's rounded corners close up square.
use crate::sketch::{block, rounded_rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::offset::{grown, Join};
use cad_kernel::topo::Brep;

const FLOOR: f64 = 4.;

pub fn build() -> Result<Brep, String> {
    let solid = block([-20., -15., 0.], [20., 15., 10.])?;
    // the pocket: its rounded outline on the floor swept up past the top
    let pocket = prism(&rounded_rectangle([-10., -7.], [10., 7.], 2., FLOOR), 0., 10. - FLOOR + 1.)?;
    grown(&combined(&solid, &pocket, Op::Cut, 0.)?, 3., Join::Round)
}
