//! `swept/box-screwed-inside-a-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: a rod (radius 5, from the origin to
//! 20w) with a sealed helical tunnel, everything the unit cube 2.5 ≤ p·u ≤ 3.5, |p·v| ≤ 0.5,
//! 2 ≤ p·w ≤ 3 passes through along the screw about the rod's axis that rises 4 a turn, from 0° to
//! 720°.
//!
//! Built as the original is, every point and direction it places at (x, y, z) placed at
//! x·u + y·v + z·w: the kernel's sweep of a solid in parts (`sweep_solid_parts`), the cube as a
//! convex polyhedron, screwed about w; the rod is cut by the parts.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid_parts, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SolidTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// The point (or direction) the upright case puts at (x, y, z), here at x·u + y·v + z·w.
fn at(x: f64, y: f64, z: f64) -> Vec3 {
    let (u, v, w) = (p(2.0, -2.0, 1.0).scale(1.0 / 3.0), p(2.0, 1.0, -2.0).scale(1.0 / 3.0),
                     p(1.0, 2.0, 2.0).scale(1.0 / 3.0));
    u.scale(x).add(v.scale(y)).add(w.scale(z))
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), at(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SolidTool::Polyhedron {
        vertices: (0..8)
            .map(|i| at(if i & 1 != 0 { 3.5 } else { 2.5 }, if i & 2 != 0 { 0.5 } else { -0.5 }, if i & 4 != 0 { 3.0 } else { 2.0 }))
            .collect(),
        faces: vec![vec![0, 2, 3, 1], vec![4, 5, 7, 6], vec![0, 1, 5, 4], vec![2, 6, 7, 3], vec![0, 4, 6, 2], vec![1, 3, 7, 5]],
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: at(0.0, 0.0, 1.0), turn: 1.0, advance: 4.0 / TAU },
        ],
        from: 0.0,
        to: (4.0 * PI),
    };
    operands.extend(sweep_solid_parts(&tool0, &motion0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
