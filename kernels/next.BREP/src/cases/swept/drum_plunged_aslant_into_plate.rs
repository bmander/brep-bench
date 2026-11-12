//! A plate pocketed from below by a drum lying on its side, pushed up aslant and stopping inside:
//! the plate from (0, −5, 0) to (20, 5, 4), less everything the drum (radius 2, its axis the line
//! y = 0, z = −5 from x = 6 to x = 14, its ends flat) passes through as it slides without turning
//! by v = 8·(0.3, 0, √(1 − 0.3²)).
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the drum, a solid of revolution
//! about the line y = 0, z = −5 whose meridian is the rectangle out to radius 2 from x = 6 to x =
//! 14, slid by v, and cut from the plate.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(0.0, -5.0, 0.0), 20.0, 10.0, 4.0)?;
    let drum = revolved_tool(p(6.0, 0.0, -5.0), p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0), &[
        Piece::Line { from: [0.0, 0.0], to: [2.0, 0.0] },
        Piece::Line { from: [2.0, 0.0], to: [2.0, 8.0] },
        Piece::Line { from: [2.0, 8.0], to: [0.0, 8.0] },
        Piece::Line { from: [0.0, 8.0], to: [0.0, 0.0] },
    ]);
    let slide = slide_by(p(0.3, 0.0, (1.0 - 0.3f64 * 0.3).sqrt()).scale(8.0));
    less_all(plate, vec![sweep_solid(&drum, &slide)?])
}
