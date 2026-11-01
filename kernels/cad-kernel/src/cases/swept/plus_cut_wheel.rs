//! A wheel, the disc of radius 40 about the vertical line through (−50, 0) from z = −8 to −2, cut
//! by a plus (arms 30 long and 8 wide along ±x and ±y, extruded from z = −10 to 0) turning about z
//! by t while the wheel turns by −2t/3, over t from 0° to 540°: what is left is what lies outside
//! the tool at every instant.
//!
//! Refused. cad-kernel has no sweep of a solid along a motion. Seen from the wheel the plus moves
//! in its own plane, turning about a centre that runs round the wheel (a roll of the circles of
//! radius 20 and 30 about the two axes), and it is a prism square to that plane through the whole
//! wheel, so the cut is the region the plus sweeps in the plane, extruded — `build::prism` would
//! extrude it. But that region is bounded by the envelopes of the plus's sides under the roll and
//! the traces of its corners (trochoids), curves cad-kernel has no form of; written as B-splines
//! fitted to points of the envelope, they would be geometry made outside the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a motion: seen from the wheel the plus rolls in \
         its plane, and the wheel's flanks are the envelope of the plus's sides under that roll \
         and the traces of its corners (trochoids), which it cannot build"
        .into())
}
