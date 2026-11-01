//! A four-armed star handle: the outline in the xz plane of four tip arcs (radius 4, centred 33.1
//! out on the axes) and four fillet arcs (radius 3, centred 15.9 out on the diagonals), each
//! tip joined to the fillets beside it by the lines tangent to both that pass between them,
//! extruded from y = 0 to y = 12 with the kernel's drafted extrusion, 2° inward: every side moves
//! in parallel to itself, the tip arcs shrinking and the fillet arcs growing about their centres.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep_draft, make_arc, make_line, BrepSolid, NurbsCurve, Vec3};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 12.0;
const DRAFT_DEGREES: f64 = 2.0;

/// The point (x, z) of the outline's plane y = 0.
fn xz([x, z]: [f64; 2]) -> Vec3 {
    p(x, 0.0, z)
}

/// The direction at `angle` in the xz plane, from +x towards +z.
fn towards(angle: f64) -> Vec3 {
    p(angle.cos(), 0.0, angle.sin())
}

/// `[x, z]` turned about the origin by `angle`, from +x towards +z.
fn turned([x, z]: [f64; 2], angle: f64) -> [f64; 2] {
    let (s, c) = angle.sin_cos();
    [c * x - s * z, s * x + c * z]
}

/// The arc of radius `r` about `centre` spanning `half` either side of the direction `middle`
/// (angles in the xz plane, from +x towards +z), run turning from +x towards +z or, `backward`,
/// the other way.
fn arc(centre: [f64; 2], r: f64, middle: f64, half: f64, backward: bool)
    -> Result<NurbsCurve, String> {
    let quarter = if backward { -FRAC_PI_2 } else { FRAC_PI_2 };
    make_arc(xz(centre), towards(middle), towards(middle + quarter), r, -half, half)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let tip = [TIP_DISTANCE, 0.0];
    let fillet = turned([FILLET_DISTANCE, 0.0], FRAC_PI_4);
    // The side from the first tip to the fillet circle beside it, tangent to both and between
    // them: its normal n has the tip's centre r₁ to one side and the fillet's r₂ to the other, so
    // n · (tip − fillet) = r₁ + r₂; of the two such lines, n the line between the centres turned
    // clockwise is the one bounding the arm.
    let (dx, dz) = (tip[0] - fillet[0], tip[1] - fillet[1]);
    let gap = dx.hypot(dz);
    let n = turned([dx / gap, dz / gap], -((TIP_RADIUS + FILLET_RADIUS) / gap).acos());
    let on_tip = [tip[0] - TIP_RADIUS * n[0], tip[1] - TIP_RADIUS * n[1]];
    let on_fillet = [fillet[0] + FILLET_RADIUS * n[0], fillet[1] + FILLET_RADIUS * n[1]];
    // Half of each arc, about its middle: the tip's outer arc about the axis, the fillet's inner
    // arc about the direction from its centre to the origin.
    let tip_half = (on_tip[1] - tip[1]).atan2(on_tip[0] - tip[0]);
    let inward = FRAC_PI_4 + PI;
    let from_inward = (on_fillet[1] - fillet[1]).atan2(on_fillet[0] - fillet[0]) - inward;
    let fillet_half = (from_inward + PI).rem_euclid(TAU) - PI;
    // The arm's other side, mirrored across its centre line.
    let mirror = |[x, z]: [f64; 2]| [x, -z];
    let mut outline = Vec::new();
    for arm in 0..4 {
        let a = FRAC_PI_2 * arm as f64;
        // Round the tip's outer arc across the axis, in along one side, round the fillet's
        // inner arc (facing the origin) the other way, out along the next arm's side to its tip.
        outline.push(arc(turned(tip, a), TIP_RADIUS, a, tip_half, false)?);
        outline.push(make_line(xz(turned(on_tip, a)), xz(turned(on_fillet, a)))?);
        outline.push(arc(turned(fillet, a), FILLET_RADIUS, a + inward, fillet_half, true)?);
        let next = a + FRAC_PI_2;
        let (from, to) = (turned(mirror(on_fillet), next), turned(mirror(on_tip), next));
        outline.push(make_line(xz(from), xz(to))?);
    }
    // The outline faces +y, so the draft runs along +y; a positive angle draws the section in.
    let draft = DRAFT_DEGREES.to_radians();
    Ok(extrude_profile_brep_draft(&outline, p(0.0, 1.0, 0.0), HEIGHT, draft, None)?)
}
