//! A ring sector grooved by a bar drafted both ways from its parting plane, turned past it.
//!
//! Through the kernel's sweep of a solid within a region (`sweep_solid_clipped`, asked for 7.5 to
//! 12.5 from the z axis, z from −1.5 to 1.5, angles −10° to 70°, which holds the block): the bar as a convex polyhedron,
//! every point a u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2° and
//! |b| ≤ 2 − |h| tan 2°, u = (1, 0, 0), v = (0, cos 20°, −sin 20°), n = (0, sin 20°, cos 20°): its
//! twelve corners (four on the crease, four at each end) and ten faces; turned about the z axis from
//! −140° to +140°; cut from the block (the rectangle 8 ≤ x ≤ 12, |z| ≤ 1 revolved from 0° to 60°).
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_solid_clipped, BooleanOperation, BrepSolid, ScrewStep,
    SolidMotion, SolidTool, SweepClip,
};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let block = revolve_profile_brep(&crate::solids::polygon(&section)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 60f64.to_radians())?;
    let lean = 20f64.to_radians();
    let u = p(1.0, 0.0, 0.0);
    let v = p(0.0, lean.cos(), -lean.sin());
    let n = p(0.0, lean.sin(), lean.cos());
    let draft = 2f64.to_radians().tan();
    let at = |a: f64, b: f64, h: f64| u.scale(a).add(v.scale(b)).add(n.scale(h));
    // corners: 4k + j, k = 0 crease, 1 top (h = 4), 2 bottom (h = −4); j round the rectangle
    let mut vertices = Vec::new();
    for h in [0.0f64, 4.0, -4.0] {
        let d = h.abs() * draft;
        for (a, b) in [(10.0 + d, -(2.0 - d)), (13.0 - d, -(2.0 - d)), (13.0 - d, 2.0 - d), (10.0 + d, 2.0 - d)] {
            vertices.push(at(a, b, h));
        }
    }
    let mut faces = vec![vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
    for j in 0..4 {
        let k = (j + 1) % 4;
        faces.push(vec![j, k, 4 + k, 4 + j]);
        faces.push(vec![j, k, 8 + k, 8 + j]);
    }
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
