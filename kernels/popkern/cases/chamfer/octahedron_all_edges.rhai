// The octahedron |x| + |y| + |z| <= 10, all twelve edges chamfered sqrt(6) on both faces in one
// chamfer, four chamfers meeting at each corner.
//
// popkern makes no polyhedron from its faces, so the octahedron is what its definition says: the
// four slabs |n.p| <= 10 for n = (1, 1, 1), (1, 1, -1), (1, -1, 1) and (-1, 1, 1) intersected,
// each slab a square on the plane through the origin square to n, reaching well past the solid
// and extruded 10/|n| each way. Each slab's two faces are two opposite faces of the octahedron.
//
// popkern's chamfer of all twelve edges runs the four at each corner on until they meet in a
// point, as it does the three at a cube's corner (chamfer/cube-all-edges): it closes no corner
// with a face, so what it makes is the mitre, chamfer/octahedron-all-edges-mitred.
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

let m = model();
let solid = octahedron(m);
solid = solid.chamfer(edges(), sqrt(6.0), #{ label: "bevels" });
m
