//! A blank machined into a dome by a five-axis program of four cutters.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid: the leaning ball-end mill turning about the trunnion's axis sweeps envelopes it would have to compute, which it cannot make from its profile sweeps".into())
}
