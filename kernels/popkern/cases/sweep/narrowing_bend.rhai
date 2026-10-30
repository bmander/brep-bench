// A hollow square duct round a quarter arc of radius 30, its section narrowing linearly with the
// angle turned, from half-width 9 to 6.
//
// Refused: a popkern sweep carries one profile along a path, its size fixed (`twist` is its only
// law), and a loft skins sections with no path to follow, so a straight loft between the two
// ends would cut across the bend. No feature narrows a section as it sweeps round the arc.
throw "popkern's sweep keeps the profile's size along the path (twist is its only law), and its \
loft follows no path: no feature narrows the section round the bend";
