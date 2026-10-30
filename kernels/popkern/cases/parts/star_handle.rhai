// A four-armed star handle: its outline in the plane y = 0, extruded 12 along y with a 2° draft.
//
// popkern's scripts extrude without draft (neither an extrude's draft angle nor the draft
// feature is bound in them yet), but the drafted solid is a loft: its section at y = 12 is the
// outline moved 12·tan 2° inward, every side and arc of one section matches one of the other,
// and the loft rules each pair straight from y = 0 to y = 12, the straight sides into planes and
// the concentric arcs into cones, leaning 2° in, as the draft would.

// The point at angle `a` (radians) and distance `r` from (cx, cy).
fn polar(cx, cy, r, a) {
    [cx + r * cos(a), cy + r * sin(a)]
}

// `q` turned by `a` (radians) about the origin.
fn turned(q, a) {
    [q[0] * cos(a) - q[1] * sin(a), q[0] * sin(a) + q[1] * cos(a)]
}

// The star's outline inset by `d`: the tip circles of radius 4 - d at 33.1 on the axes and the
// fillet circles of radius 3 + d at 15.9 on the diagonals, joined by the lines tangent to each
// tip circle and the fillet circle beside it, passing between them. On the xz plane (u = x,
// v = z), whose sketches extrude towards -y; xz_at(-y) is the plane at y.
fn outline(m, d, y, label) {
    let rt = 4.0 - d;
    let rf = 3.0 + d;
    let tx = 33.1;
    let f = 15.9 / sqrt(2.0);
    // the line's normal n meets the centres' offset c_tip - c_fillet at the angle whose cosine is
    // (rt + rf) / |offset|; it touches the tip circle at c_tip - rt·n and the fillet circle at
    // c_fillet + rf·n
    let n = atan(-f, tx - f) - acos((rt + rf) / hypot(tx - f, f));
    let tip = polar(tx, 0.0, -rt, n);
    let fillet = polar(f, f, rf, n);
    // arm 0's points, from its tip arc's start below the x axis, round to the next arm's
    let points = [
        [tip[0], -tip[1]],              // the tip arc's start
        [tx + rt, 0.0],                 // through the arc's outermost point
        tip,                            // the tip arc's end
        fillet,                         // the side's end on the fillet circle
        polar(f, f, -rf, PI() / 4.0),   // through the fillet arc's point nearest the origin
        [fillet[1], fillet[0]],         // the fillet arc's end, its mirror in the diagonal
    ];
    let p = m.profile(xz_at(-y)).path(points[0][0], points[0][1]);
    for k in 0..4 {
        let a = PI() / 2.0 * k.to_float();
        let side = turned(points[3], a);
        let next = turned(points[0], a + PI() / 2.0);
        p = p.arc_to(turned(points[1], a), turned(points[2], a))
            .line_to(side[0], side[1])
            .arc_to(turned(points[4], a), turned(points[5], a))
            .line_to(next[0], next[1]);
    }
    p.close().sketch(label)
}

let m = model();
let base = outline(m, 0.0, 0.0, "base");
let top = outline(m, 12.0 * tan(2.0 * PI() / 180.0), 12.0, "top");
m.loft([base, top], #{ label: "drafted" });
m
