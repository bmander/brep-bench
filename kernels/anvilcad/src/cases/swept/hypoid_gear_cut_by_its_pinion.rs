//! A hypoid gear's tooth space cut by its pinion.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid: the cutter, carried by two motions at once relative to the blank, sweeps envelopes of its faces that it would have to compute itself".into())
}
