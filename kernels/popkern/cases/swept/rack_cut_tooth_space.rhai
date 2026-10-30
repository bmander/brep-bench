// A gear blank with one tooth space cut by a rack tooth rolled round it.
//
// Refused: popkern has no sweep of a solid (its sweeps carry a profile along a path), and this
// one's shape does not follow from its other operations by elementary geometry: the space's flanks
// are the envelope of the tooth's flanks (involutes) and its root the curve its tip corners trace.
// A wall the script would compute itself, fitting curves to where the tool touches its own sweep,
// is geometry made outside the kernel.
throw "popkern has no sweep of a solid, and this sweep's wall is an envelope: the space's flanks \
are the envelope of the tooth's flanks (involutes) and its root the curve its tip corners \
trace";
