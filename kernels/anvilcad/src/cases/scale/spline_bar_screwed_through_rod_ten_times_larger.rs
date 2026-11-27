//! A rod threaded by a bar whose section is a spline, ten times larger.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid and no helical curve: a body screwed along an axis sweeps envelopes of its surfaces under the screw, which it cannot make from its profile sweeps".into())
}
