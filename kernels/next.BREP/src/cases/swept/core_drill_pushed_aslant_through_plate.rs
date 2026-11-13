//! The plate from (−10, −10, 0) to (10, 10, 4), less everything a core drill passes through as it
//! slides by 24 along its own leaning axis u = (sin 30°, 0, cos 30°) through (0, 0, 2): the tube
//! between 1.5 and 2.5 from that axis, from 18 to 6 before (0, 0, 2) where it starts.
//!
//! Through the kernel's whole sweep of a solid (`sweep_solid`): the core drill, a solid of
//! revolution about its leaning axis whose meridian is the rectangle between 1.5 and 2.5 from the
//! axis, from 18 to 6 before (0, 0, 2), slid by 24 along that axis, and cut from the plate.
use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, slide_by, Piece};
use crate::Refusal;
use brep_kernel::{make_box_brep, sweep_solid, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let lean = 30f64.to_radians();
    let (u, out) = (p(lean.sin(), 0.0, lean.cos()), p(lean.cos(), 0.0, -lean.sin()));
    let plate = make_box_brep(p(-10.0, -10.0, 0.0), 20.0, 20.0, 4.0)?;
    let drill = revolved_tool(p(0.0, 0.0, 2.0), u, out, &[
        Piece::Line { from: [1.5, -18.0], to: [2.5, -18.0] },
        Piece::Line { from: [2.5, -18.0], to: [2.5, -6.0] },
        Piece::Line { from: [2.5, -6.0], to: [1.5, -6.0] },
        Piece::Line { from: [1.5, -6.0], to: [1.5, -18.0] },
    ]);
    less_all(plate, vec![sweep_solid(&drill, &slide_by(u.scale(24.0)))?])
}
