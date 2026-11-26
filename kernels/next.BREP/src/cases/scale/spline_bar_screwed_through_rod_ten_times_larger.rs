//! `swept/spline-bar-screwed-through-rod` ten times larger: a rod (radius 50, z from 0 to 200)
//! threaded by a bar whose section is a closed periodic cubic B-spline, carried along a screw.
//!
//! Built as the original is, every length times 10: through the kernel's sweep of a solid
//! (`sweep_solid_within`), asked for the bar (the kernel's periodic B-spline on the seven control
//! points in the plane y = 0, from y = −80 to 80) along the screw about the z axis that rises 60 a
//! turn, from −90° to +1350°, within 52 of the z axis from z = −0.5 to 200.5; cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, make_periodic_bspline, sweep_clip_less, sweep_solid_within, BooleanOperation, BrepSolid, ScrewStep,
    SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 50.0, 200.0)?;
    let poles = [(35.0, 0.0), (45.0, -15.0), (80.0, -15.0), (120.0, -10.0), (120.0, 10.0), (80.0, 15.0), (45.0, 15.0)].map(|(x, z)| p(x, 0.0, z));
    let bar = SweptTool::Bar {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 1.0, 0.0),
        section: vec![make_periodic_bspline(3, &poles)?],
        from: -80.0,
        to: 80.0,
        draft: 0.0,
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 60.0 / TAU }],
        from: (-90.0f64).to_radians(),
        to: 1350.0f64.to_radians(),
    };
    let clip = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 52.0],
        axial: [-0.5, 200.5],
        angle: None,
    };
    let mut operands = vec![rod];
    // The rod less the sweep, built directly where the kernel can (its clip
    // the rod itself); else the sweep within the larger clip, subtracted.
    let rod_clip = SweepClip { radius: [0.0, 50.0], axial: [0.0, 200.0], ..clip.clone() };
    if let Some(mut bodies) = sweep_clip_less(&bar, &screw, &rod_clip)? {
        if bodies.len() == 1 {
            return Ok(bodies.remove(0));
        }
    }
    operands.extend(sweep_solid_within(&bar, &screw, &clip)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
