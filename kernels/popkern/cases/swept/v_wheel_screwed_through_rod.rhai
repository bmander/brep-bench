// A rod threaded by a sharp V wheel carried along a screw about the rod's axis.
//
// Refused: popkern has no sweep of a solid (its sweeps carry a profile along a path), and this
// one's shape does not follow from its other operations by elementary geometry: the thread's flanks
// are the envelope of the wheel's cones along the screw. A wall the script would compute itself,
// fitting curves to where the tool touches its own sweep, is geometry made outside the kernel.
throw "popkern has no sweep of a solid, and this sweep's wall is an envelope: the thread's flanks \
are the envelope of the wheel's cones along the screw";
