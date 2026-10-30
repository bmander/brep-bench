// The solid the box from the origin to (10, 6, 4) fills as it slides, without turning, along
// (8, 5, 3).
//
// workaround: no sweep of a solid
// popkern has no sweep of a solid. The box slid along a line is the sum of its edges' three
// vectors e1 = (10, 0, 0), e2 = (0, 6, 0), e3 = (0, 0, 4) and the slide v = (8, 5, 3): the convex
// polyhedron with a pair of parallel faces square to the cross product of each two of the four,
// six slabs. A slab square to n holds the points p with n·p between the sums of the negative and
// of the positive n·g over the four vectors g. The three slabs square to the axes make the block
// from the origin to (18, 11, 7); the other three, square to e1 × v, e2 × v and e3 × v, are each
// a block on a plane square to n, through the slab's middle, reaching well past the solid and
// extruded half the slab's thickness each way.
fn dot(a, b) { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
fn cross(a, b) { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }

let m = model();
let e = [[10.0, 0.0, 0.0], [0.0, 6.0, 0.0], [0.0, 0.0, 4.0]];
let v = [8.0, 5.0, 3.0];
let swept = m.profile(xy()).rect(18.0, 11.0).at(9.0, 5.5).sketch("axes").extrude(blind(7.0));
for i in 0..3 {
    let n = cross(e[i], v);
    let length = sqrt(dot(n, n));
    let lo = 0.0;
    let hi = 0.0;
    for g in [e[0], e[1], e[2], v] {
        let s = dot(n, g);
        if s < 0.0 { lo += s; } else { hi += s; }
    }
    let middle = (lo + hi) / 2.0 / length / length;
    let slab = m.tool(|m| m.profile(plane_at([n[0] * middle, n[1] * middle, n[2] * middle], n))
        .rect(100.0, 100.0).sketch(`slab_${i}`).extrude(mid_plane((hi - lo) / 2.0 / length)));
    swept = swept.intersect(slab, #{ label: `slid_${i}` });
}
m
