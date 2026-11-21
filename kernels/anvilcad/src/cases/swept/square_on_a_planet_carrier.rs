//! A square bar carried round on a planet gear.
use anvil_brep::Solid;

pub fn build() -> Result<Solid, String> {
    Err("anvil-brep has no sweep of a solid: the cutter, carried by two motions at once relative to the blank, sweeps envelopes of its faces that it would have to compute itself".into())
}
