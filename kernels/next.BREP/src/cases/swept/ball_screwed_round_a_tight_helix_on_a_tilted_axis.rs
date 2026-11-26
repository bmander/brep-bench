//! `swept/ball-screwed-round-a-tight-helix` turned rigidly so that its axis runs along
//! w = (1, 2, 2)/3, with u = (2, −2, 1)/3 and v = (2, 1, −2)/3: the solid a ball of radius 2 fills
//! as its centre runs two turns of the helix of radius 1.5 about the line through the origin along
//! w, from 1.5u, rising 3 a turn. Built as the original is (see it for the argument), every point
//! and direction it places at (x, y, z) placed at x·u + y·v + z·w: the kernel's sweep of the ball
//! by the screw about w where it builds it; else the cut discs lofted by eighths, the end balls and
//! the cylinder of radius 0.5 about the axis from the origin to 6w, united.
use crate::solids::p;
use crate::solids::swept::{planar, revolved_tool, Piece};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, loft_profile_brep, make_cylinder_brep, make_sphere_brep, sweep_solid, BooleanOperation, BrepSolid, NurbsCurve, ScrewStep, SolidMotion, Vec3};
use std::f64::consts::{PI, TAU};

const RADIUS: f64 = 1.5;
const BALL: f64 = 2.0;
const CUT: f64 = 1.6;

/// The point the upright case puts at (x, y, z), here at x·u + y·v + z·w.
fn at(x: f64, y: f64, z: f64) -> Vec3 {
    let (u, v, w) = (p(2.0, -2.0, 1.0).scale(1.0 / 3.0), p(2.0, 1.0, -2.0).scale(1.0 / 3.0),
                     p(1.0, 2.0, 2.0).scale(1.0 / 3.0));
    u.scale(x).add(v.scale(y)).add(w.scale(z))
}

pub fn build() -> Result<BrepSolid, Refusal> {
    // The kernel's sweep of the ball itself, where it builds the screwed ball past its fold (its
    // tube and end balls, trimmed where the arc's own distance says); the union below otherwise.
    let ball = revolved_tool(at(RADIUS, 0.0, -BALL), at(0.0, 0.0, 1.0), at(1.0, 0.0, 0.0), &[
        Piece::Arc { centre: [0.0, BALL], radius: BALL, start: -PI / 2.0, end: PI / 2.0 },
        Piece::Line { from: [0.0, 2.0 * BALL], to: [0.0, 0.0] },
    ]);
    // the screw about the line through the origin along w, rising 3 a turn
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: at(0.0, 0.0, 1.0), turn: 1.0, advance: 3.0 / TAU }],
        from: 0.0,
        to: 2.0 * TAU,
    };
    if let Ok(solid) = sweep_solid(&ball, &screw) {
        return Ok(solid);
    }
    let rise = 3.0 / TAU;
    let speed = (RADIUS * RADIUS + rise * rise).sqrt();
    let centre = |t: f64| at(RADIUS * t.cos(), RADIUS * t.sin(), rise * t);
    // The chord's ends on the disc, in (along N, along B), and the arc's angle there.
    let half = (BALL * BALL - CUT * CUT).sqrt();
    let corner = half.atan2(CUT);
    let section = |t: f64| -> Result<Vec<NurbsCurve>, Refusal> {
        let tangent = at(-RADIUS * t.sin(), RADIUS * t.cos(), rise).scale(1.0 / speed);
        let normal = at(-t.cos(), -t.sin(), 0.0);
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
    let up = at(0.0, 0.0, 1.0);
    parts.push(make_sphere_brep(centre(0.0), BALL, up)?);
    parts.push(make_sphere_brep(centre(2.0 * TAU), BALL, up)?);
    parts.push(make_cylinder_brep(p(0.0, 0.0, 0.0), up, 0.5, 6.0)?);
    Ok(boolean_operation_nary(&parts, BooleanOperation::Union)?)
}
