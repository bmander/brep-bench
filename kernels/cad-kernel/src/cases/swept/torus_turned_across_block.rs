//! A ring sector (radii 8 to 12 about z, z from −1 to 1, polar angle 0° to 60°) grooved by a torus
//! of radii 2.5 and 2 centred at (5, 0, 0), its axis leaning 25° from z towards +y, turned rigidly
//! about z from −140° to +140°.
//!
//! Refused. cad-kernel has no sweep of a solid along a turn. The cutter's points stay within about
//! 64° of the x axis about z, so over a turn of ±140° every point of the block at a distance ρ from
//! the z axis and a height z that some point of the cutter has is passed through: the groove is
//! the cutter's shadow on a half-plane through the z axis, revolved (`build::revolve` turns a
//! planar profile). But that shadow is not a profile cad-kernel can draw. The torus is the union
//! of the balls of radius 2 about the circle through its tube's centres, and each ball's shadow is
//! the disc of radius 2 about where its centre stands in (ρ, z); leaning, that circle's image is a
//! curve that is no line, circle or conic, and the shadow's boundary is its parallel curve at 2 —
//! the envelope of the discs, the curve where the turn's velocity lies tangent to the torus. It
//! would have to be computed and fitted, and that is geometry made outside the kernel.
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    Err("cad-kernel has no sweep of a solid along a turn: revolve turns only a planar profile, and \
         the profile this groove needs — the leaning cutter's shadow on a half-plane through the z \
         axis — is bounded by the envelope of the torus under the turn (the parallel curve at 2 of \
         its tube's centre circle's image), which it cannot build"
        .into())
}
