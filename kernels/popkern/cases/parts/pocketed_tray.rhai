// An open tray on its back, opening towards -y. Sketches on the xz plane (u = x, v = z) extrude
// towards -y, and xz_at(d) is the plane y = -d.
let m = model();
// 1. the block from (-36, 0, -24) to (36, 16, 24)
let tray = m.profile(xz_at(-16.0)).rect(72.0, 48.0).sketch("block").extrude(blind(16.0));
// 2. the pocket, x -33..33 and z -21..21, from the floor at y = 13 out past the face y = 0
let pocket = m.tool(|m| m.profile(xz_at(-13.0)).rect(66.0, 42.0)
    .sketch("pocket").extrude(blind(14.0)));
tray = tray.subtract(pocket);
// 3. four standoffs on the floor, each a tube along y from y = 5 to 13, outer radius 5 and inner
//    radius 2: its hole stops at the floor it stands on
for c in [[26.0, 14.0], [-26.0, 14.0], [-26.0, -14.0], [26.0, -14.0]] {
    let standoff = m.tool(|m| {
        let post = m.profile(xz_at(-13.0)).circle(5.0).at(c[0], c[1])
            .sketch("post").extrude(blind(8.0));
        let bore = m.tool(|m| m.profile(xz_at(-14.0)).circle(2.0).at(c[0], c[1])
            .sketch("bore").extrude(blind(10.0)));
        post.subtract(bore)
    });
    tray = tray.union(standoff);
}
m
