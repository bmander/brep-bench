//! A gear blank, the disc of radius 22 about y from y = 0 to 6, with one tooth space cut by a rack
//! tooth — the prism on the trapezoid (18, ±0.842856), (24, ±3.026666) in (x, z), from y = −2 to
//! 8 — whose pitch line x = 20 rolls without slipping on the pitch circle of radius 20, from 60°
//! on one side of (20, 0, 0) to 60° on the other.
//!
//! Refused. cad-kernel has no sweep of a solid along a motion, and this one rolls. The tooth is a
//! prism square to the plane of the roll and runs through the blank, so the cut is the region the
//! trapezoid sweeps in that plane, extruded — and `build::prism` would extrude it. But that region
//! is bounded by the envelopes of the tooth's flanks (involutes of the base circle of radius
//! 20·cos 20°) and the traces of its tip corners (trochoids), curves cad-kernel has no form of: a
//! profile is lines, arcs and B-splines, and an involute or a trochoid written as a B-spline from
//! its formula is the envelope computed outside the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a rolling motion: the tooth space is the \
         trapezoid's planar sweep extruded, but its flanks are the envelope of the rack's flanks \
         (involutes of the base circle) and its root the trace of the tip corners (trochoids), \
         curves cad-kernel cannot build except by fitting them outside it"
        .into())
}
