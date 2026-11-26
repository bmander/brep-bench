//! A wheel carried along a cubic Bezier by its moving frame, cut from a block.
//! Uses the kernel's continuous solid sweep and boolean subtraction.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, sweep_solid_along,
    BooleanOperation, BrepSolid, FrameCurve, OutlinePiece, SolidTool};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let ring = SolidTool::Revolved {
        axis_origin: p(0.0, 0.0, 0.5),
        axis_direction: p(0.0, 1.0, 0.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc {
            centre: [1.5, 0.0], radius: 0.5, start: 0.0, end: TAU,
        }],
    };
    let path = FrameCurve {
        control: [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0),
                  p(20.0, 20.0, 0.0), p(30.0, 20.0, 0.0)],
        up: p(0.0, 0.0, 1.0),
    };
    let swept = sweep_solid_along(&ring, &path)?;
    let block = make_box_brep(p(-6.0, -6.0, -8.0), 42.0, 32.0, 8.0)?;
    Ok(boolean_operation_nary(&[block, swept], BooleanOperation::Subtract)?)
}
