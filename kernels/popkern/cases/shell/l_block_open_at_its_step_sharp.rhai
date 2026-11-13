// An L-shaped block shelled inward open at its step, the wall rising past the opening kept. The
// outline through (0, 0), (20, 0), (20, 20), (15, 20), (15, 5) and (0, 5), extruded 20 along +z,
// shelled inward to a wall 1 thick, open at the face y = 5, the corners sharp.
// Stood on its side (the outline in the xz plane, u = x, v = z, extruded 20 across it), so that
// the opening is the face facing +z at height 5: the same solid turned, its volume and area the
// same.
let m = model();
let b = m.profile(xz()).polygon([[0.0, 0.0], [20.0, 0.0], [20.0, 20.0], [15.0, 20.0], [15.0, 5.0],
    [0.0, 5.0]]).sketch("l_outline").extrude(blind(20.0));
b = b.shell(faces().facing("z").at_height(5.0), 1.0,
    #{ join: "intersection", label: "hollow" });
m
