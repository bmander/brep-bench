//! A rod threaded by a bar along a path whose lead grows as it climbs.
//!
//! Through the kernel's sweep of a solid along a turn whose rise grows (`sweep_solid_rising`): the
//! bar, the box 4 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 1, as a convex polyhedron; turned by θ about the z axis
//! it has risen 6s + 0.3s², s = θ/2π, so the rise is (6/2π)θ + (0.3/4π²)θ², for θ from −90° to
//! 1260°; asked for within 5 of the z axis (the rod, which the turn maps onto itself, so the kernel
//! may sweep only the bar's part that reaches it); cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_rising, BooleanOperation, BrepSolid, RisingTurn, SolidTool,
    SweepClip,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let bar = SolidTool::Polyhedron {
        vertices: (0..8)
            .map(|i| p(if i & 1 != 0 { 12.0 } else { 4.0 }, if i & 2 != 0 { 8.0 } else { -8.0 }, if i & 4 != 0 { 1.0 } else { -1.0 }))
            .collect(),
        faces: vec![vec![0, 2, 3, 1], vec![4, 5, 7, 6], vec![0, 1, 5, 4], vec![2, 6, 7, 3], vec![0, 4, 6, 2], vec![1, 3, 7, 5]],
    };
    let motion = RisingTurn {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        rise: [6.0 / TAU, 0.3 / (TAU * TAU)],
        from: (-90f64).to_radians(),
        to: 1260f64.to_radians(),
    };
    let rod_region = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.0],
        axial: [-0.4, 20.4],
        angle: None,
    };
    let swept = sweep_solid_rising(&bar, &motion, &rod_region)?;
    Ok(boolean_operation_nary(&[rod, swept], BooleanOperation::Subtract)?)
}
