//! A ring sector whose floor is cut by the flat end of a leaning bar turned past it.
//!
//! Through the kernel's sweep of a solid within a region (`sweep_solid_clipped`, asked for 7.5 to
//! 12.5 from the z axis, z from −1.5 to 1.5, angles −10° to 70°, which holds the block), the bar as a convex polyhedron: its
//! end, the rectangle {0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2} of the plane through
//! (10, 0, 0) along u = (cos 20°, 0, −sin 20°) and v = (0, 1, 0), carried 3 along n = (sin 20°, 0,
//! cos 20°); turned about the z axis from −140° to +140°; cut from the block (the rectangle 8 ≤ x ≤ 12,
//! |z| ≤ 1 revolved from 0° to 60°).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid_clipped, BooleanOperation, BrepSolid, ScrewStep,
    SolidMotion, SolidTool, SweepClip,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&crate::solids::polygon(&section)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 60f64.to_radians())?;
    let (c, s) = (5f64.to_radians().cos(), 5f64.to_radians().sin());
    let lean = 20f64.to_radians();
    let u = p(lean.cos(), 0.0, -lean.sin());
    let v = p(0.0, 1.0, 0.0);
    let n = p(lean.sin(), 0.0, lean.cos());
    // a corner (a', b') of the turned rectangle, at height h along n
    let corner = |a: f64, b: f64, h: f64| {
        let (x, y) = (a * c - b * s, a * s + b * c);
        p(10.0, 0.0, 0.0).add(u.scale(x)).add(v.scale(y)).add(n.scale(h))
    };
    let mut vertices = Vec::new();
    for h in [0.0, 3.0] {
        for (a, b) in [(0.0, -2.0), (3.0, -2.0), (3.0, 2.0), (0.0, 2.0)] {
            vertices.push(corner(a, b, h));
        }
    }
    let faces = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![0, 1, 5, 4], vec![1, 2, 6, 5], vec![2, 3, 7, 6], vec![3, 0, 4, 7]];
    let motion = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 0.0 }],
        from: (-140f64).to_radians(),
        to: 140f64.to_radians(),
    };
    let around_block = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10f64).to_radians(), 70f64.to_radians()]),
    };
    let swept = sweep_solid_clipped(&SolidTool::Polyhedron { vertices, faces }, &motion, &around_block)?;
    Ok(boolean_operation_nary(&[block, swept], BooleanOperation::Subtract)?)
}
