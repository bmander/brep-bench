//! `swept/ball-screwed-round-a-tight-helix` ten times larger: the solid a ball of radius 20 fills as
//! its centre runs two turns of the helix of radius 15 about the z axis rising 30 a turn:
//! everything within 20 of the helix's arc. Built as the original is (see it for the argument),
//! every length times 10: the kernel's sweep of the ball where it builds the screwed ball past its
//! fold; else the cut discs (cut at 16, short of the radius of curvature ≈ 16.52) lofted by
//! eighths, the end balls and the cylinder of radius 5 about the axis from z = 0 to 60, united.
use crate::solids::p;
use crate::solids::swept::{planar, revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, loft_profile_brep, make_cylinder_brep, make_sphere_brep, sweep_solid, BooleanOperation, BrepSolid, NurbsCurve};
use std::f64::consts::{PI, TAU};

const RADIUS: f64 = 15.0;
const BALL: f64 = 20.0;
const CUT: f64 = 16.0;

pub fn build() -> Result<BrepSolid, Refusal> {
    // The kernel's sweep of the ball itself, where it builds the screwed ball past its fold (its
    // tube and end balls, trimmed where the arc's own distance says); the union below otherwise.
    let ball = revolved_tool(p(RADIUS, 0.0, -BALL), p(0.0, 0.0, 1.0), p(1.0, 0.0, 0.0), &[
        Piece::Arc { centre: [0.0, BALL], radius: BALL, start: -PI / 2.0, end: PI / 2.0 },
        Piece::Line { from: [0.0, 2.0 * BALL], to: [0.0, 0.0] },
    ]);
    if let Ok(solid) = sweep_solid(&ball, &turn_about_z(0.0, 2.0 * TAU, 30.0 / TAU)) {
        return Ok(solid);
    }
    let rise = 30.0 / TAU;
    let speed = (RADIUS * RADIUS + rise * rise).sqrt();
    let centre = |t: f64| p(RADIUS * t.cos(), RADIUS * t.sin(), rise * t);
    // The chord's ends on the disc, in (along N, along B), and the arc's angle there.
    let half = (BALL * BALL - CUT * CUT).sqrt();
    let corner = half.atan2(CUT);
    let section = |t: f64| -> Result<Vec<NurbsCurve>, Refusal> {
        let tangent = p(-RADIUS * t.sin(), RADIUS * t.cos(), rise).scale(1.0 / speed);
        let normal = p(-t.cos(), -t.sin(), 0.0);
        Ok(planar(
            &[
                Piece::Arc { centre: [0.0, 0.0], radius: BALL, start: corner, end: TAU - corner },
                Piece::Line { from: [CUT, -half], to: [CUT, half] },
            ],
            centre(t),
            normal,
            tangent.cross(normal),
        )?)
    };
    let mut parts = Vec::new();
    for k in 0..8 {
        let sections: Vec<_> = (0..33).map(|i| section(PI / 2.0 * (k as f64 + i as f64 / 32.0))).collect::<Result<_, _>>()?;
        parts.push(loft_profile_brep(&sections)?);
    }
    let up = p(0.0, 0.0, 1.0);
    parts.push(make_sphere_brep(centre(0.0), BALL, up)?);
    parts.push(make_sphere_brep(centre(2.0 * TAU), BALL, up)?);
    parts.push(make_cylinder_brep(p(0.0, 0.0, 0.0), up, 5.0, 60.0)?);
    Ok(boolean_operation_nary(&parts, BooleanOperation::Union)?)
}
