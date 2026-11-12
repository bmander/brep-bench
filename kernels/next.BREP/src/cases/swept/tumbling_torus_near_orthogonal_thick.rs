//! A torus tumbled about nearly perpendicular axes, through the kernel's sweep of a solid of
//! revolution tumbled by two turns about skew axes (`sweep_solid_tumbled`): the torus centred at
//! (2.02, 0, 1), axis along z, the circle through its tube's centres of radius 1, tube radius 0.16;
//! at roll t turned by 1.01t about the line through (2, 0, 0) along z, then by −t about the line
//! through the origin along (cos 89°, sin 89° cos 1°, sin 89° sin 1°); t from −180° to 180°. The
//! turn is a little over a full revolution, so the two half-times are swept apart and joined.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, sweep_solid_tumbled, BooleanOperation, BrepSolid, OutlinePiece, SolidTool, TwoTurns};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = SolidTool::Revolved {
        axis_origin: p(2.02, 0.0, 1.0),
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [1.0, 0.0], radius: 0.16, start: 0.0, end: TAU }],
    };
    let (l, a) = (1f64.to_radians(), 89f64.to_radians());
    let half = |from: f64, to: f64| TwoTurns {
        axes: [
            (p(2.0, 0.0, 0.0), p(0.0, 0.0, 1.0)),
            (p(0.0, 0.0, 0.0), p(a.cos(), a.sin() * l.cos(), a.sin() * l.sin())),
        ],
        rates: [1.01, -1.0],
        from: from.to_radians(),
        to: to.to_radians(),
    };
    let first = sweep_solid_tumbled(&torus, &half(-180.0, 0.0))?;
    let second = sweep_solid_tumbled(&torus, &half(0.0, 180.0))?;
    Ok(boolean_operation_nary(&[first, second], BooleanOperation::Union)?)
}
