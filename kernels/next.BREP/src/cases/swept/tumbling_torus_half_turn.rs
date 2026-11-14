//! A torus of tube radius 0.16 tumbled through half a roll, through the kernel's sweep of a solid of
//! revolution tumbled by two turns about skew axes (`sweep_solid_tumbled`): the torus centred at
//! (3, 0, 2), axis along z, the circle through its tube's centres of radius 1, tube radius 0.16; at
//! roll t turned by 0.375t about the line through (2, 0, 0) along z, then by −t about the line through
//! (0, h cos 30°, h sin 30°), h = 0.7071, along (cos 20°, sin 20° cos 30°, sin 20° sin 30°); t from −90° to 90°.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{sweep_solid_tumbled, BrepSolid, OutlinePiece, SolidTool, TwoTurns};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let torus = SolidTool::Revolved {
        axis_origin: p(3.0, 0.0, 2.0),
        axis_direction: p(0.0, 0.0, 1.0),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [1.0, 0.0], radius: 0.16, start: 0.0, end: TAU }],
    };
    let (l, a, h) = (30f64.to_radians(), 20f64.to_radians(), 0.7071);
    let tumble = TwoTurns {
        axes: [
            (p(2.0, 0.0, 0.0), p(0.0, 0.0, 1.0)),
            (p(0.0, h * l.cos(), h * l.sin()), p(a.cos(), a.sin() * l.cos(), a.sin() * l.sin())),
        ],
        rates: [0.375, -1.0],
        from: (-90f64).to_radians(),
        to: 90f64.to_radians(),
    };
    Ok(sweep_solid_tumbled(&torus, &tumble)?)
}
