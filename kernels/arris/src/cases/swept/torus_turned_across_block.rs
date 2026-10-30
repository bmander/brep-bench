//! A sector of a ring about the z axis (radius 8 to 12, z from −1 to 1, polar angle 0° to 60°),
//! grooved by a torus (radii 2.5 and 2, centred at (5, 0, 0), its axis leaning 25° from z towards
//! +y) turned rigidly about the z axis from −140° to +140°.
//!
//! Refused: arris has no sweep of a solid. A turn about the z axis would make this a revolution:
//! across the block, which the cutter passes wholly through, its sweep is the region the torus
//! covers in the half plane through the z axis as it turns, revolved. But the cutter's axis is
//! skew to the z axis, so that region's boundary is the outline of the torus seen round the axis,
//! an envelope (the curve on the torus where the turn's velocity is tangent to it, carried round),
//! none of the torus's own circles. arris cannot build it; it would have to be computed outside
//! the kernel and fitted.
use crate::Refusal;
use arris::topo::{Body, Model};

pub fn build(_: &mut Model) -> Result<Body, Refusal> {
    Err("arris has no sweep of a solid; the groove's wall is the envelope of the leaning torus \
         turned about the z axis, whose meridian outline it cannot build"
        .into())
}
