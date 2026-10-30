//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical groove ground round it
//! by a torus of radii 6 and 2, centred at (11.5, 0, 0) with its axis (0, cos h, −sin h), h =
//! atan(2π·5.5 / 30), screwed about z — rising 30 a turn — from a turn of −108° to +348°.
//!
//! Refused. cad-kernel has no sweep of a solid along a screw. What it offers, and why none of it
//! builds this:
//!
//! - `build::canal` sweeps a ball along a screw, the ball's disc square to the helix carried along
//!   it; but the wheel is a torus, and the union of the tubes its tube's balls sweep (one for each
//!   point of the circle through the tube's centres) is bounded by their envelope, no one tube.
//! - A face family (`family::family`) can carry a planar section along the screw, and the groove
//!   is the screw's own: its section square to z is the same at every height, turned, so the
//!   groove is that section screwed. But that section is the wheel's shadow along the screw, and
//!   its boundary is the curve where the screw's velocity lies tangent to the torus (the wheel's
//!   characteristic, carried round): an envelope, which cad-kernel does not compute, and which
//!   this file may not compute for it.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a screw motion: its canal screws only a ball, \
         and the groove's wall is the envelope of the torus under the screw (the section a face \
         family could screw along z is bounded by the wheel's characteristic curve), which it \
         cannot build"
        .into())
}
