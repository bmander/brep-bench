// The solid a torus fills as it turns about one axis, seen from a frame turning about another.
//
// Refused: popkern has no sweep of a solid (its sweeps carry a profile along a path), and this
// one's shape does not follow from its other operations by elementary geometry: no line of the
// torus stays fixed, so its boundary is an envelope of no simpler kind. A wall the script would
// compute itself, fitting curves to where the tool touches its own sweep, is geometry made outside
// the kernel.
throw "popkern has no sweep of a solid, and this sweep's wall is an envelope: no line of the \
torus stays fixed, so its boundary is an envelope of no simpler kind";
