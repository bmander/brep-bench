//! Refused, as `swept/spline-bar-screwed-through-rod` is, and for its reason, which holds at every
//! size: the spline bar's side meets the screw square-on along a curve found only numerically on
//! its rulings, so the groove's walls are an envelope no planar section's screw sweep makes.
use crate::Refusal;
use crate::solids::Body;

pub fn build() -> Result<Body, Refusal> {
    crate::cases::swept::spline_bar_screwed_through_rod::build()
}
