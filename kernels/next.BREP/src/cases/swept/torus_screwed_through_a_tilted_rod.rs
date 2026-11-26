//! `swept/torus-screwed-through-rod` turned rigidly so that the rod's axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: a rod (radius 5, from the origin to
//! 20w) grooved by a torus wheel (radii 6 and 2 about 11.5u, its axis cos h·v − sin h·w) carried
//! along the screw about the rod's axis rising 30 a turn, from −108° to +348°.
//!
//! Built as the original is, every point and direction it places at (x, y, z) placed at
//! x·u + y·v + z·w: the kernel's sweep of a solid (`sweep_clip_less`, else `sweep_solid_within`
//! inside the rod's cylinder of radius 5.5 from −1 to 21 along w), cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_clip_less, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool, Vec3,
};
use std::f64::consts::TAU;

/// The point (or direction) the upright case puts at (x, y, z), here at x·u + y·v + z·w.
fn at(x: f64, y: f64, z: f64) -> Vec3 {
    let (u, v, w) = (p(2.0, -2.0, 1.0).scale(1.0 / 3.0), p(2.0, 1.0, -2.0).scale(1.0 / 3.0),
                     p(1.0, 2.0, 2.0).scale(1.0 / 3.0));
    u.scale(x).add(v.scale(y)).add(w.scale(z))
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), at(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let h = (TAU * 5.5 / 30.0).atan();
    let wheel = SweptTool::Revolved {
        axis_origin: at(11.5, 0.0, 0.0),
        axis_direction: at(0.0, h.cos(), -h.sin()),
        radial: at(-1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [6.0, 0.0], radius: 2.0, start: 0.0, end: TAU }],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep {
            axis_origin: p(0.0, 0.0, 0.0),
            axis_direction: at(0.0, 0.0, 1.0),
            turn: 1.0,
            advance: 30.0 / TAU,
        }],
        from: (-108.0f64).to_radians(),
        to: 348.0f64.to_radians(),
    };
    let around_rod = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: at(0.0, 0.0, 1.0),
        reference: at(1.0, 0.0, 0.0),
        radius: [0.0, 5.5],
        axial: [-1.0, 21.0],
        angle: None,
    };
    let mut operands = vec![rod];
    // The rod less the sweep, built directly where the kernel can (its clip
    // the rod itself); else the sweep within the larger clip, subtracted.
    let rod_clip = SweepClip { radius: [0.0, 5.0], axial: [0.0, 20.0], ..around_rod.clone() };
    if let Some(mut bodies) = sweep_clip_less(&wheel, &screw, &rod_clip)? {
        if bodies.len() == 1 {
            return Ok(bodies.remove(0));
        }
    }
    operands.extend(sweep_solid_within(&wheel, &screw, &around_rod)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
