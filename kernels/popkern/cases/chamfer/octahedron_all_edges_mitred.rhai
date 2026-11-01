// The octahedron |x| + |y| + |z| <= 10, all twelve edges chamfered sqrt(6) on both faces, each
// edge its own chamfer, so that each runs on to the chamfers made before it and the four at a
// corner meet in a point.
//
// popkern makes no polyhedron from its faces, so the octahedron is what its definition says: the
// four slabs |n.p| <= 10 for n = (1, 1, 1), (1, 1, -1), (1, -1, 1) and (-1, 1, 1) intersected,
// each slab a square on the plane through the origin square to n, reaching well past the solid
// and extruded 10/|n| each way. Each slab's two faces are two opposite faces of the octahedron.
//
// popkern declines this. It chamfers an edge after one beside it at a corner (sharing a face),
// but not after the one opposite it there: the two chamfers meet along a line through the
// corner's last point, such as x = 8, y = 0 between (8, 0, -2) and (8, 0, 2) for the edges from
// (10, 0, 0) to (0, 10, 0) and to (0, -10, 0), and the second consumes the sides the first left
// across the other two faces at the corner, which popkern's chamfer refuses ("keeps no length"),
// handing the solid back unchanged. Of the four edges at a corner, the third chamfered is always
// opposite one before it, so every order meets this. (One chamfer of all twelve edges together
// makes this mitre: see chamfer/octahedron-all-edges. The case asks for the edges one at a time,
// which popkern can select.)
fn octahedron(m) {
    let normals = [[1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [1.0, -1.0, 1.0], [-1.0, 1.0, 1.0]];
    let solid = m.profile(plane_at([0.0, 0.0, 0.0], normals[0])).rect(60.0, 60.0)
        .sketch("slab_0").extrude(mid_plane(10.0 / sqrt(3.0)));
    for i in 1..4 {
        let slab = m.tool(|m| m.profile(plane_at([0.0, 0.0, 0.0], normals[i])).rect(60.0, 60.0)
            .sketch(`slab_${i}`).extrude(mid_plane(10.0 / sqrt(3.0))));
        solid = solid.intersect(slab, #{ label: `octahedron_${i}` });
    }
    solid
}

// The corner on axis `axis` at `s` times 10, such as (10, 0, 0).
fn corner(axis, s) {
    let p = [0.0, 0.0, 0.0];
    p[axis] = s * 10.0;
    p
}

fn dot(a, b) { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

// The point a fraction `t` of the way from `a` to `b`.
fn between(a, b, t) {
    [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1]), a[2] + t * (b[2] - a[2])]
}

let m = model();
let solid = octahedron(m);
// Each edge is found on the body as it stands, by its two ends. A label would not do: it follows
// an edge through at most one chamfer, as a chamfer's names keep only their direct ancestors, and
// not through one that trims the edge across a corner. The chamfer of the edge from a to b lies
// in the plane n.p = 8, n = (a + b)/10 (on the face x + y + z = 10, the chamfer x + y = 8 is the
// line z = 2, sqrt(6) from the edge), and what is left of an edge is the part of it on the solid's
// side of every chamfer made before it.
let made = [];
for axis in 0..3 {
    let next = (axis + 1) % 3;
    for s in [1.0, -1.0] {
        for t in [1.0, -1.0] {
            let a = corner(axis, s);
            let b = corner(next, t);
            let from = 0.0;
            let to = 1.0;
            for n in made {
                let na = dot(n, a);
                let nb = dot(n, b);
                if na > 8.0 && nb < 8.0 { from = max(from, (na - 8.0) / (na - nb)); }
                if nb > 8.0 && na < 8.0 { to = min(to, (8.0 - na) / (nb - na)); }
            }
            solid = solid.chamfer(
                edges().at_vertex(between(a, b, from)).at_vertex(between(a, b, to)), sqrt(6.0),
                #{ label: `bevel_${made.len()}` });
            made.push([(a[0] + b[0]) / 10.0, (a[1] + b[1]) / 10.0, (a[2] + b[2]) / 10.0]);
        }
    }
}
m
