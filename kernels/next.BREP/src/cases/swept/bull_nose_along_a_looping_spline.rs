//! A block grooved by an upright bull-nose mill run along a cubic Bézier curve that loops over
//! itself and turns tighter than the mill.
//!
//! Through the kernel's sweep of a solid carried by a frame along a curve (`sweep_solid_along`):
//! the curve is the cubic Bézier (0, 0, 0), (30, 20, 0), (−10, 20, 0), (20, 0, 0), the frame's up
//! the z axis; the mill, drawn in the frame at the curve's start, is a solid of revolution about the
//! upright through (0, 0, −1.5), its meridian the flat end out to 1.5, the quarter circle of
//! radius 0.5 about (1.5, 0.5), the side at 2 up to 6.5 and the flat top; cut from the block, the
//! box from (−10, −10, −8) to (30, 25, 0). The kernel builds the upright mill's sweep from the
//! planar distance to the curve, its folded inner wall and the island inside the loop included.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_box_brep, sweep_solid_along, BooleanOperation, BrepSolid, FrameCurve, OutlinePiece, SolidTool,
};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(-10.0, -10.0, -8.0), 40.0, 35.0, 8.0)?;
    let mill = SolidTool::Revolved {
        axis_origin: p(0.0, 0.0, -1.5),
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: [0.0, 0.0], to: [1.5, 0.0] },
            OutlinePiece::Arc { centre: [1.5, 0.5], radius: 0.5, start: -FRAC_PI_2, end: 0.0 },
            OutlinePiece::Line { from: [2.0, 0.5], to: [2.0, 6.5] },
            OutlinePiece::Line { from: [2.0, 6.5], to: [0.0, 6.5] },
            OutlinePiece::Line { from: [0.0, 6.5], to: [0.0, 0.0] },
        ],
    };
    let path = FrameCurve {
        control: [p(0.0, 0.0, 0.0), p(30.0, 20.0, 0.0), p(-10.0, 20.0, 0.0), p(20.0, 0.0, 0.0)],
        up: p(0.0, 0.0, 1.0),
    };
    let swept = sweep_solid_along(&mill, &path)?;
    Ok(boolean_operation_nary(&[block, swept], BooleanOperation::Subtract)?)
}
