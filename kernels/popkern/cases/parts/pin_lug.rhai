// A plate from (0, -5, 0) to (80, 5, 40): its outline in the xz plane (u = x, v = z) extruded 5
// each way along y. A pin hole of radius 8 on the line x = 60, z = 20, cut by a cylinder running
// past both faces.
let m = model();
let lug = m.profile(xz()).rect(80.0, 40.0).at(40.0, 20.0).sketch("plate").extrude(mid_plane(5.0));
let pin = m.tool(|m| m.profile(xz()).circle(8.0).at(60.0, 20.0)
    .sketch("pin").extrude(mid_plane(6.0)));
lug = lug.subtract(pin);
m
