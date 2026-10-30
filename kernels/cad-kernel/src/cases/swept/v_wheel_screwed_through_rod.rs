//! A rod, the cylinder of radius 5 about z from z = 0 to 20, with a helical V groove ground round
//! it by a wheel with a sharp 60° V rim — the triangle (4, 0, 0), (6.5, 0, 0) ± 2.5·tan 30°·along
//! turned about the axis through (14, 0, 0) along (0, cos h, −sin h), h = atan(2π·4.5 / 30) —
//! screwed about z, rising 30 a turn, from a turn of −144° to +384°.
//!
//! Refused. cad-kernel has no sweep of a solid along a screw. What it offers, and why none of it
//! builds this:
//!
//! - `build::canal` sweeps only a ball along a screw; the wheel is a ring of two cones and a
//!   cylinder.
//! - A face family (`family::family`) can carry a planar section along the screw, and the groove
//!   is the screw's own: its section square to z is the same at every height, turned. But that
//!   section is the wheel's shadow along the screw, bounded by where the screw's velocity lies
//!   tangent to each flank's cone (the cones' characteristic curves, carried round) and by the
//!   trace of the sharp rim: an envelope, which cad-kernel does not compute, and which this file
//!   may not compute for it.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a screw motion: its canal screws only a ball, \
         and the V groove's flanks are the envelope of the wheel's conical flanks under the screw \
         (the section a face family could screw along z is bounded by their characteristic \
         curves), which it cannot build"
        .into())
}
