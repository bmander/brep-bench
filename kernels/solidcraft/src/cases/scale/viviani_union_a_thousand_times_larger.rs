//! The ball and cylinder of `scale/viviani-common-a-thousand-times-larger`: their union.
use super::viviani_common_a_thousand_times_larger::{ball, cylinder};
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    fuse(ball()?, cylinder()?)
}
