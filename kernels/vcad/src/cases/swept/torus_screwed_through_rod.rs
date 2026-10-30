//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground round it by a torus (radii 6 and 2, centred at (11.5, 0, 0), its axis leaning by the
//! helix angle) carried along a screw about the z axis that rises 30 a turn, from −108° to +348°.
//!
//! Refused: vcad has no sweep of a solid, and none along a screw motion (`Solid::sweep` moves a
//! sketch profile along a curve, a `Helix` among them). The groove's wall is the envelope of the
//! screwed torus, the curve on it where the screw's velocity is tangent to it swept along the
//! screw; its section is none of the torus's own circles, so no profile vcad could sweep along the
//! helix is known without computing that characteristic curve outside the kernel.
use vcad_kernel::Solid;

pub fn build() -> Result<Solid, crate::Refusal> {
    Err("vcad has no sweep of a solid along a screw motion; the groove's wall is the envelope of \
         the screwed torus, whose section is none of the torus's circles and would have to be \
         computed outside the kernel"
        .into())
}
