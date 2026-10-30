//! A four-armed star in the xz plane (tips r 4 at 33.1, fillets r 3 at 15.9) drafted 2° to y = 12.
use cad_kernel::build::{drafted, Profile, ProfileEdge};
use cad_kernel::geom::{Frame, V};
use cad_kernel::topo::Brep;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI, TAU};

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.;
const HEIGHT: f64 = 12.;
const DRAFT_DEGREES: f64 = 2.;

/// The point `(x, z)` of the outline's plane, y = 0.
fn xz([x, z]: [f64; 2]) -> V {
    [x, 0., z]
}

/// `p` turned about the origin by `angle`, from +x towards +z.
fn turned([x, z]: [f64; 2], angle: f64) -> [f64; 2] {
    let (s, c) = angle.sin_cos();
    [c * x - s * z, s * x + c * z]
}

/// The arc of the circle about `centre` of radius `r` from `p` to `q` (either way round) that
/// crosses the ray from the centre at `through` (an angle from +x towards +z).
fn arc(normal: V, centre: [f64; 2], r: f64, p: [f64; 2], q: [f64; 2], through: f64) -> ProfileEdge {
    let frame = Frame::new(xz(centre), normal, [1., 0., 0.]);
    let angle = |a: V| { let l = frame.local(a); l[1].atan2(l[0]) };
    let mid = angle(xz([centre[0] + through.cos(), centre[1] + through.sin()]));
    let (ap, aq) = (angle(xz(p)), angle(xz(q)));
    // the span counter-clockwise about the normal from one end to the other that holds `through`
    let ccw = |a: f64, b: f64| [a, a + (b - a).rem_euclid(TAU)];
    let holds = |[a, b]: [f64; 2]| (mid - a).rem_euclid(TAU) < b - a;
    let span = if holds(ccw(ap, aq)) { ccw(ap, aq) } else { ccw(aq, ap) };
    ProfileEdge::Arc { frame, r, span: Some(span) }
}

pub fn build() -> Result<Brep, String> {
    // the outline faces +y, so the draft runs along +y
    let normal = [0., 1., 0.];
    let tip = [TIP_DISTANCE, 0.];
    let fillet = turned([FILLET_DISTANCE, 0.], FRAC_PI_4);
    // the side from the first tip to the fillet circle beside it, tangent to both and between
    // them: its normal n has the tip's centre r₁ to one side and the fillet's r₂ to the other, so
    // n · (tip − fillet) = r₁ + r₂; of the two such lines, n the line between the centres turned
    // clockwise is the one in the arm's wedge
    let (dx, dz) = (tip[0] - fillet[0], tip[1] - fillet[1]);
    let gap = dx.hypot(dz);
    let n = turned([dx / gap, dz / gap], -((TIP_RADIUS + FILLET_RADIUS) / gap).acos());
    let on_tip = [tip[0] - TIP_RADIUS * n[0], tip[1] - TIP_RADIUS * n[1]];
    let on_fillet = [fillet[0] + FILLET_RADIUS * n[0], fillet[1] + FILLET_RADIUS * n[1]];
    // the arm's other side, mirrored across its centre line
    let mirror = |[x, z]: [f64; 2]| [x, -z];
    let mut edges = Vec::new();
    for arm in 0..4 {
        let a = FRAC_PI_2 * arm as f64;
        let (tip_before, tip_after) = (turned(mirror(on_tip), a), turned(on_tip, a));
        let (fillet_before, fillet_after) = (turned(mirror(on_fillet), a), turned(on_fillet, a));
        let next_fillet = turned(mirror(on_fillet), a + FRAC_PI_2);
        // out along one side, round the tip's outer arc across the axis, in along the other side,
        // round the fillet's inner arc (facing the origin) to the next arm
        edges.push(ProfileEdge::Line { a: xz(fillet_before), b: xz(tip_before) });
        edges.push(arc(normal, turned(tip, a), TIP_RADIUS, tip_before, tip_after, a));
        edges.push(ProfileEdge::Line { a: xz(tip_after), b: xz(fillet_after) });
        let fillet_centre = turned(fillet, a);
        let inward = a + FRAC_PI_4 + PI;
        edges.push(arc(normal, fillet_centre, FILLET_RADIUS, fillet_after, next_fillet, inward));
    }
    let outline = Profile { origin: [0., 0., 0.], normal, loops: vec![edges], names: Vec::new() };
    // the section at y = h is the outline moved h·tan 2° into its material: straight sides
    // inward, tip arcs smaller and fillet arcs larger about their centres
    drafted(&outline, 0., HEIGHT, DRAFT_DEGREES.to_radians())
}
