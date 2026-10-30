// A hollow square duct turned a quarter turn about the z axis, from +x towards +y: its section,
// the square 18 × 18 centred 30 out in the xz plane (u = x, v = z) less the square hole 14 × 14,
// is the same all the way round.
//
// A popkern revolve through part of a turn makes NURBS faces, and the boolean of two such
// quarter turns (outside less hole) does not finish; a full turn makes exact planes and
// cylinders. So the whole ring is revolved, outside and hole, and the quarter from +x to +y is
// kept: its two ends are the planes x = 0 and y = 0, as the duct's are.
let m = model();
let z = axis_at([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
let duct = m.profile(xz()).rect(18.0, 18.0).at(30.0, 0.0)
    .sketch("outside").revolve(z, full_circle());
let bore = m.tool(|m| m.profile(xz()).rect(14.0, 14.0).at(30.0, 0.0)
    .sketch("inside").revolve(z, full_circle()));
duct = duct.subtract(bore);
let quadrant = m.tool(|m| m.profile(xy_at(-10.0)).rect(40.0, 40.0).at(20.0, 20.0)
    .sketch("quadrant").extrude(blind(20.0)));
duct = duct.intersect(quadrant);
m
