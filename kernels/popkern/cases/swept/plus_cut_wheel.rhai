// A wheel cut by a plus-shaped tool that turns in mesh with it, as a gear shaper cuts.
//
// Refused: popkern has no sweep of a solid (its sweeps carry a profile along a path), and this
// one's shape does not follow from its other operations by elementary geometry: the tool's sweep,
// seen from the turning wheel, is bounded by the envelope of its sides. A wall the script would
// compute itself, fitting curves to where the tool touches its own sweep, is geometry made outside
// the kernel.
throw "popkern has no sweep of a solid, and this sweep's wall is an envelope: the tool's sweep, \
seen from the turning wheel, is bounded by the envelope of its sides";
