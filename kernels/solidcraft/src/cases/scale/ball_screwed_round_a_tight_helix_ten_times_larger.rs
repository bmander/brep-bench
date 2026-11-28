//! Refused, as `swept/ball-screwed-round-a-tight-helix` is, and for its reason, which holds at
//! every size: the helix (radius 15, rising 30 a turn) still turns tighter than the ball (radius
//! 20), its curvature's radius 16.5 against 20, so the screw sweep of the ball's forward-moving
//! great disc runs through itself and the solid's boundary is an envelope trimmed where it meets
//! itself.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    crate::cases::swept::ball_screwed_round_a_tight_helix::build()
}
