//! The ball and cylinder of `scale/viviani-common-a-thousand-times-larger`: the ball less the
//! cylinder, pinched to the point where they touch.
use super::viviani_common_a_thousand_times_larger::{ball, cylinder};
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    cut(ball()?, cylinder()?)
}
