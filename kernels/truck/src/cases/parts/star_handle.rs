//! "A four-armed star handle, as on a radiator valve: a flat star outline in the xz plane, extruded
//! 12 along y with a draft. [The outline, in the plane y = 0:] four tip circles of radius 4,
//! centred 33.1 from the origin on the axes; four fillet circles of radius 3, centred 15.9 from the
//! origin on the diagonals. Each arm's two straight sides are tangent lines, each to the arm's tip
//! circle and to the fillet circle beside it, passing between the two circles. [Going round: the
//! outer arc of a tip circle, a side, the inner arc of a fillet circle, a side, and so on.] The
//! extrusion, with a 2° draft: the solid runs from y = 0 to y = 12, and its section at y = h is the
//! outline moved h·tan 2° inward: every straight side moves inward parallel to itself, each tip
//! arc's radius shrinks and each fillet arc's radius grows by h·tan 2°, the circles' centres
//! staying where they are."
//!
//! truck has no draft (and no offset), so the drafted sides are lofted: the outline is drawn at
//! y = 0 and again at y = 12 with the radii the description gives there, and `try_wire_homotopy`
//! joins each edge of one to the same edge of the other by straight lines. Two parallel sides
//! span a plane, and two arcs on one centre through the same angle span the cone between them.
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};
use truck_modeling::*;

const TIP_CENTRE: f64 = 33.1; // from the origin, on the axes
const TIP_RADIUS: f64 = 4.0;
const FILLET_CENTRE: f64 = 15.9; // from the origin, on the diagonals
const FILLET_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 12.0; // along y
const DRAFT: f64 = 2.0; // degrees

/// The star's outline in the plane `y`, its tip circles of radius `tip` and its fillet circles of
/// radius `fillet`, run from +x towards +z round the y axis (clockwise seen from +y), so that a
/// face it bounds looks along -y.
fn outline(y: f64, tip: f64, fillet: f64) -> Wire {
    // the point (x, z) of the plane turned `k` quarter turns from +x towards +z
    let at = |k: usize, (x, z): (f64, f64)| {
        let (s, c) = (k as f64 * FRAC_PI_2).sin_cos();
        Point3::new(c * x - s * z, y, s * x + c * z)
    };
    // the side from the tip circle on +x to the fillet circle on the diagonal beside it, the line
    // between them: its unit normal n, turned from the line of centres by the angle whose cosine
    // is (tip + fillet) over the distance between the centres, so that the tip circle lies on
    // one side and the fillet circle on the other
    let tip_centre = Vector2::new(TIP_CENTRE, 0.0);
    let fillet_centre = Vector2::new(FILLET_CENTRE, FILLET_CENTRE) * FRAC_1_SQRT_2;
    let between = tip_centre - fillet_centre;
    let u = between.normalize();
    let (s, c) = ((tip + fillet) / between.magnitude()).acos().sin_cos();
    let n = Vector2::new(c * u.x + s * u.y, c * u.y - s * u.x);
    let p = tip_centre - n * tip; // where the side touches the tip circle
    let q = fillet_centre + n * fillet; // and the fillet circle
    // each arm's tip arc crosses its axis, and the fillet arc after it crosses the diagonal on
    // the side facing the origin
    let tip_middle = (TIP_CENTRE + tip, 0.0);
    let inner = (FILLET_CENTRE - fillet) * FRAC_1_SQRT_2;
    let fillet_middle = (inner, inner);

    // per arm: the tip arc's ends either side of the axis, then the fillet arc's ends either side
    // of the diagonal (the arm's sides mirrored in the axis and in the diagonal)
    let v = builder::vertices((0..4).flat_map(|k| {
        [at(k, (p.x, -p.y)), at(k, (p.x, p.y)), at(k, (q.x, q.y)), at(k, (q.y, q.x))]
    }));
    (0..4)
        .flat_map(|k| {
            let w = |i: usize| &v[(4 * k + i) % v.len()];
            [
                builder::circle_arc(w(0), w(1), at(k, tip_middle)),
                builder::line(w(1), w(2)),
                builder::circle_arc(w(2), w(3), at(k, fillet_middle)),
                builder::line(w(3), w(4)),
            ]
        })
        .collect()
}

pub fn build() -> crate::Built {
    let inset = HEIGHT * DRAFT.to_radians().tan();
    let base = outline(0.0, TIP_RADIUS, FILLET_RADIUS);
    let top = outline(HEIGHT, TIP_RADIUS - inset, FILLET_RADIUS + inset);
    // the sides, joined from the top down so that they look outward
    let mut shell: Shell = builder::try_wire_homotopy(&top, &base)?;
    // the ends, on the very edges the sides run to: the base looks along -y, the top along +y
    shell.push(builder::try_attach_plane(&[base])?);
    shell.push(builder::try_attach_plane(&[top.inverse()])?);
    let handle = Solid::try_new(vec![shell])?;
    Ok(handle.into())
}
