// A curved block grooved by a leaning torus turned past it about the block's axis.
//
// Refused: popkern has no sweep of a solid (its sweeps carry a profile along a path), and this
// one's shape does not follow from its other operations by elementary geometry: the cutter's axis
// is skew to the turn's, so the groove's wall is the torus's envelope, none of its own circles. A
// wall the script would compute itself, fitting curves to where the tool touches its own sweep, is
// geometry made outside the kernel.
throw "popkern has no sweep of a solid, and this sweep's wall is an envelope: the cutter's axis \
is skew to the turn's, so the groove's wall is the torus's envelope, none of its own \
circles";
