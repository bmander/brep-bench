// A duct along y from the square A B C D at y = 0 to the diamond A' B' C' D' at y = 40, each
// corner running straight to the corner with its letter: a loft of the two sections, corner to
// corner, in the xz plane (u = x, v = z; xz_at(-40) is the plane y = 40).
let m = model();
let square = m.profile(xz())
    .polygon([[10.0, -10.0], [-10.0, -10.0], [-10.0, 10.0], [10.0, 10.0]]).sketch("square");
let diamond = m.profile(xz_at(-40.0))
    .polygon([[0.0, -12.0], [-12.0, 0.0], [0.0, 12.0], [12.0, 0.0]]).sketch("diamond");
m.loft([square, diamond], #{ label: "duct" });
m
