// A cube of side 10 from the origin, all twelve edges chamfered 2 on both faces, each edge its
// own chamfer, so that each runs on to the chamfers made before it and the three at a corner
// meet in a point.
//
// popkern declines this: the second chamfer at a corner runs on to the first, and in doing so
// consumes the side the first left on the third face there (on the face x = 0, after the edge
// along x at y = z = 0, the side from (0, 2, 0) to (0, 0, 2)), which shrinks to the point the
// two meet in; popkern's chamfer refuses a side it would leave with no length ("keeps no
// length"), handing the solid back unchanged. Every two edges at a corner of a cube share a
// face, so every order meets this at its first corner chamfered twice. (One chamfer of all
// twelve edges together does make this mitre, the corners run to a point: see
// chamfer/cube-all-edges. The case asks for the edges one at a time, which popkern can select.)
//
// Each edge is found on the body as it stands. A label would not do: it follows an edge through
// one chamfer, but not two, as a chamfer's names keep only their direct ancestors. The four
// edges along x are apart, so each is chamfered while it still runs between its corners. Each
// edge along y or z is chamfered after one at each of its corners, which cut it back 2 at each
// end, and is found by the end it has nearer the origin.
let m = model();
let cube = m.profile(xy()).rect(10.0, 10.0).at(5.0, 5.0).sketch("cube").extrude(blind(10.0));
let axes = ["x", "y", "z"];
let n = 0;
for axis in 0..3 {
    for u in [0.0, 10.0] {
        for v in [0.0, 10.0] {
            let end = [u, v];
            end.insert(axis, if axis == 0 { 0.0 } else { 2.0 });
            cube = cube.chamfer(edges().direction(axes[axis]).at_vertex(end), 2.0,
                #{ label: `bevel_${n}` });
            n += 1;
        }
    }
}
m
